These are the underlying pallets needed for the smart contract environment. On each pallet I give a brief description of it's intended purpose and function. 
I put these together for a Voting Smart Contract implementation in Substrate. 

Despite Ethereum being more popular for Smart Contract development I chose to go with Substrate's VRF (Verifiable Random Function) over ETH's RANDAO because of it's unique way of securing the network. It enables unpredictable validator groupings by shifting validators so quickly that the cost to attempt to compromise the network would far outweigh the benefits. 

Given a network of 20 Validators (10 primary, 10 reserve), which are assigned randomly on the minute, you would have a 1/3^(10+10) chance of the network being compromised given a maximum of 1/3 bad actors. Assuming no external players interfering, it would take about 660 years to takeover the network. 

So far I personally haven't yet seen another blockchain that enables random, near instant validator shifts with modular and self contained units. Also I personally prefer Substrate's Stake-Weighted voting system which gives more nuance to the voting process by allowing you to express how much you're invested in an outcome rather than just a binary yes/no.  

