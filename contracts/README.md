# PoZK contracts

```
npm run build # build all contracts
npm run abi   # build all ABI files in ../public/ABI
```

## Deploy
### Locally
```
npx hardhat node
npm run deploy
```

### Chains
```
npm run deploy --network xxx
```

## Test
`npm run test`

## Contracts
- **Addresses**: Record all contracts address
- **Token**: Main token(ERC20) in PoZK network
- **Vesting**: Token lock status and unlock period
- **Controller**: Common controller accounts for player/miner
- **Epoch**: "Block time" in the network
- **Stake**: Player/Miner/Game staking status
- **Reward**: Network Rewards Distribution
- **GameMarket**: Manage all registered games
- **TaskMarket**: Manage all proof tasks

## License

This project is licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html).
