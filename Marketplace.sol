// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.0 <0.9.0;

import "./IERC721.sol";
import "./Address.sol";

/**
 * @dev Order info data structure
 * @param orderId The identifier of the order, incrementing uint256 starting from 0
 * @param orderType The type of the order, 1 is sale order, 2 is auction order, 3 is splittable order
 * @param orderState The state of the order, 1 is open, 2 is filled, 3 is cancelled
 * @param collection The token contract address representing collection
 * @param tokenId The token type placed in the order
 * @param quoteToken The address of the token accepted as payment for the order
 * @param price The price asked for the order (minimum bidding price for auction order)
 * @param endTime The end time of the auction (only meaningful for auction order)
 * @param seller The address of the seller that created the order
 * @param buyer The address of the buyer of the order
 * @param bids The number of bids placed on the order (only meaningful for auction orders)
 * @param lastBidder The address of the last bidder that bids on the order (only meaningful for auction orders)
 * @param lastBid The last bid price on the order (only meaningful for auction orders)
 * @param createTime The timestamp of the order creation
 * @param updateTime The timestamp of last order info update
 */
struct OrderInfo {
    uint256 orderId;
    uint256 orderType;
    uint256 orderState;
    address collection;
    uint256 tokenId;
    address quoteToken;
    uint256 price;
    uint256 endTime;
    address seller;
    address buyer;
    uint256 bids;
    address lastBidder;
    uint256 lastBid;
    uint256 createTime;
    uint256 updateTime;
}

/**
 * @dev Account info data structure
 * @param index The index of the account, incrementing uint256 starting from 0
 * @param account The address of the account
 * @param orderCount The number of orders created by the account
 * @param openCount The number of currently open orders created by the account
 * @param filledCount The number of orders filled by the account
 * @param joinTime The timestamp when the account first joins in Marketplace
 * @param lastActionTime The timestamp of the accountʻs last action in Marketplace
 */
struct AccountInfo {
    uint256 index;
    address account;
    uint256 orderCount;
    uint256 openCount;
    uint256 filledCount;
    uint256 joinTime;
    uint256 lastActionTime;
}

contract Marketplace {
    using Address for address;

    bytes constant MARKET_DATA = bytes("NFT Marketplace");

    OrderInfo[] private orders;
    uint256[] private openOrders;
    mapping(uint256 => uint256) private openOrderToIndex;

    address[] private accounts;
    mapping(address => AccountInfo) private addressToAccount;

    /**
     * @dev MUST emit when a new sale order is created in Market.
     * The `seller` argument MUST be the address of the seller who created the order.
     * The `orderId` argument MUST be the id of the order created.
     * The `collection` argument MUST be the address of the token contract.
     * The `tokenId` argument MUST be the token type placed on sale.
     * The `quoteToken` argument MUST be the address of the token accepted as payment for the order.
     * The `price` argument MUST be the fixed price asked for the sale order.
     */
    event OrderForSale(address seller, uint256 indexed orderId, address indexed collection, uint256 indexed tokenId, address quoteToken, uint256 price);

    /**
     * @dev MUST emit when a new auction order is created in Market.
     * The `seller` argument MUST be the address of the seller who created the order.
     * The `orderId` argument MUST be the id of the order created.
     * The `collection` argument MUST be the address of the token contract.
     * The `tokenId` argument MUST be the token type placed on auction.
     * The `quoteToken` argument MUST be the address of the token accepted as payment for the auction.
     * The `minPrice` argument MUST be the minimum starting price for the auction bids.
     * The `endTime` argument MUST be the time for ending the auction.
     */
    event OrderForAuction(address seller, uint256 indexed orderId, address indexed collection, uint256 indexed tokenId, address quoteToken, uint256 minPrice, uint256 endTime);

    /**
     * @dev MUST emit when a bid is placed on an auction order.
     * The `seller` argument MUST be the address of the seller who created the order.
     * The `bidder` argument MUST be the address of the bidder who made the bid.
     * The `orderId` argument MUST be the id of the order been bid on.
     * The `price` argument MUST be the price of the bid.
     */
    event OrderBid(address indexed seller, address indexed bidder, uint256 indexed orderId, uint256 price);

    /**
     * @dev MUST emit when an order is filled.
     * The `seller` argument MUST be the address of the seller who created the order.
     * The `buyer` argument MUST be the address of the buyer in the fulfilled order.
     * The `collection` argument MUST be the address of the token contract.
     * The `orderId` argument MUST be the id of the order fulfilled.
     * The `quoteToken` argument MUST be the address of the token used as payment for the fulfilled order.
     * The `price` argument MUST be the price of the fulfilled order.
     */
    event OrderFilled(address seller, address indexed buyer, address indexed collection, uint256 indexed orderId, address quoteToken, uint256 price);

    /**
     * @dev MUST emit when an order is canceled.
     * @dev Only an open sale order or an auction order with no bid yet can be canceled
     * The `seller` argument MUST be the address of the seller who created the order.
     * The `orderId` argument MUST be the id of the order canceled.
     */
    event OrderCanceled(address indexed seller, uint256 indexed orderId);

    /**
     * @dev MUST emit when an order has its price changed.
     * @dev Only an open sale order or an auction order with no bid yet can have its price changed.
     * @dev For sale orders, the fixed price asked for the order is changed.
     * @dev for auction orders, the minimum starting price for the bids is changed.
     * The `seller` argument MUST be the address of the seller who created the order.
     * The `orderId` argument MUST be the id of the order with the price change.
     * The `oldPrice` argument MUST be the original price of the order before the price change.
     * The `newPrice` argument MUST be the new price of the order after the price change.
     */
    event OrderPriceChanged(address indexed seller, uint256 indexed orderId, uint256 oldPrice, uint256 newPrice);

    /**
     * @dev MUST emit when ERC721 token is transferred to this contract.
     */
    event ERC721TokenReceived(address indexed operator, address indexed from, address indexed tokenAddress, uint256 tokenId, bytes data);

    /**
     * @notice Create a new order for sale at a fixed price
     * @param _collection The token contract address
     * @param _tokenId The token placed on sale
     * @param _quoteToken The address of the token accepted as payment for the order.
     * @param _price The fixed price asked for the sale order
     */
    function createOrderForSale(address _collection, uint256 _tokenId, address _quoteToken, uint256 _price) external returns (uint256) {
        require(IERC721(_collection).ownerOf(_tokenId) == msg.sender, "caller is not the owner of token");
        require(IERC721(_collection).getApproved(_tokenId) == address(this), "token is not aprroved by seller");
        require(_price > 0, "price cannot be zero");
        
        IERC721(_collection).safeTransferFrom(msg.sender, address(this), _tokenId, MARKET_DATA);
        uint256 orderId = _createOrder(1, _collection, _tokenId, _quoteToken, _price, 0);
        emit OrderForSale(msg.sender, orderId, _collection, _tokenId, _quoteToken, _price);

        return orderId;
    }

    /**
     * @notice internal createOrder utility method
     */
    function _createOrder(uint256 _orderType, address _collection, uint256 _tokenId, address _quoteToken, uint256 _price, uint256 _endTime) internal returns(uint256) {
        OrderInfo memory newOrder;
        newOrder.orderId = orders.length;
        newOrder.orderType = _orderType;
        newOrder.orderState = 1;
        newOrder.collection = _collection;
        newOrder.tokenId = _tokenId;
        newOrder.quoteToken = _quoteToken;
        newOrder.price = _price;
        newOrder.endTime = _endTime;
        newOrder.seller = msg.sender;
        newOrder.createTime = block.timestamp;
        newOrder.updateTime = block.timestamp;

        orders.push(newOrder);
        openOrderToIndex[newOrder.orderId] = openOrders.length;
        openOrders.push(newOrder.orderId);

        return newOrder.orderId;
    }
}
