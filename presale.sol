//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";

contract Presale is Ownable {
    using SafeMath for uint;

    // map of whitelisted addresses
    mapping (address => bool) public whitelist;

    // map of the total tokens purchased from an address
    mapping (address => uint) public purchasedTokens;

    // token received after participating in the presale
    IERC20 public tokensForSale;

    // asset for users to deposit eg, usdc, usdt...
    IERC20 public tokensBeingRaised;

    // min and max allocation 
    uint public constant MIN_ALLOCATION = 100 ether;
    uint public constant MAX_ALLOCATION = 1000 ether;

    // hardcap of the raise
    uint public constant HARDCAP = 400000 ether;

    // amounts of tokens per dollar received: 1 / 0.27; 0.27 is the presale price for 1 token
    uint public constant TOKENPERUSD = 3.7037037037 ether;

    // total amount raised in usd value
    uint public totalRaised = 0 ether;

    // all the addresses that partcipated
    address[] internal participating;

    // total percentage distributed
    uint totalPercentageDistributed = 0;

    // is the presale active? currently not
    bool isActive = false;

    // is the whitelist active? currently yes, once turned off, you cant turn on whitelist anymore
    bool isWhitelist = true;

    constructor(address _tokensForSale, address _tokensBeingRaised) {
        // the token address of the token being raised
        tokensForSale = IERC20(_tokensForSale);
        tokensBeingRaised = IERC20(_tokensBeingRaised);
    }

    function addToWhitelist(address[] memory accounts) public onlyOwner {
        require(isActive == false, "Presale already started");
        // adds a list of addresses to the whitelist
        for (uint256 i = 0; i < accounts.length; i++) {
            whitelist[accounts[i]] = true;
        }
    }

    function startPresale() public onlyOwner {
        require(isActive == false, "startPresale :: presale has already started");
        // starts the presale
        isActive = true;
    }

    function stopPresale() public onlyOwner {
        require(isActive, "stopPresale :: presale is already stopped");
        // stops the presale
        isActive = false;
    }

    function stopWhiteList() public onlyOwner {
        require(isWhitelist, "stopWhiteList :: whitelist is already stopped");
        // turns off whitelist
        isWhitelist = false;
    }

    function isWhitelisted(address account) external view returns (bool) {
        // checks if a wallet is whitelisted or not
        return whitelist[account];
    }
  
    function buy(uint amount) public {
        require(isActive, "buy :: presale is not active yet");
        require(amount >= MIN_ALLOCATION && amount <= MAX_ALLOCATION, "buy :: amount is not between the min and max allocation");
        require(tokensBeingRaised.balanceOf(msg.sender) >= amount, "buy :: you do not own enough raise tokens");
        require(totalRaised <= HARDCAP, "buy :: hardcap has been reached");
        require((totalRaised + amount) <= HARDCAP, "buy :: you will be going over the hardcap");
        require(purchasedTokens[msg.sender] + amount <= MAX_ALLOCATION, "buy :: you cant buy more than the max allocation");

        if (isWhitelist) {
            require(whitelist[msg.sender], "buy :: you are not whitelisted");
        }

        // receives the tokens being raised from the address
        tokensBeingRaised.transferFrom(msg.sender, address(this), amount);
        
        // incremements the total amount raised
        totalRaised = totalRaised.add(amount);

        // a list of all the participating wallets 
        participating.push(msg.sender);

        // the amount of purchased tokens from a wallet in usd value
        purchasedTokens[msg.sender] = purchasedTokens[msg.sender].add(amount);
    }

    function distributeTokens(uint percentageOfAmountOwed) public onlyOwner {
        require((totalPercentageDistributed + percentageOfAmountOwed) <= 100, "distributeTokens :: already distributed 100% of tokens");

        // distributes the tokens based on how much a user allocated, (logic may be wrong, please double check)
        for (uint i = 0; i < participating.length; i++) {
            // say you purchase $100 and want to distribute 10%
            // 100 * 3.7037037037 = 370 SEEDED TOKENS
            // 370 / 100 = 3.7
            // 3.7 * 10 = 37 SEEDED TOKENS
            tokensForSale.transfer(participating[i], purchasedTokens[participating[i]].mul(TOKENPERUSD).div(100).mul(percentageOfAmountOwed));
        }
    }

    function withdrawFunds() external onlyOwner {
        // withdraw all the funds in the token raised
        tokensBeingRaised.transfer(msg.sender, tokensBeingRaised.balanceOf(address(this)));
    }
    
    function withdrawUnsoldTokens() external onlyOwner {
        // withdraw any unsold tokens being offered for sale
        tokensForSale.transfer(msg.sender, tokensForSale.balanceOf(address(this)));
    }
}