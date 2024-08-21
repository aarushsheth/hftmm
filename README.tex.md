# High Frequency Trading in a Limit Order Book

Assume there are a finite number of time steps indexed by $t=0,1, \ldots, T$. Assume the market-maker always shows a bid price and ask price (at each time $t$ ) along with the associated bid shares and ask shares on the OB. Also assume, for ease of exposition, that the market-maker can add or remove bid/ask shares from the OB costlessly. We use the following notation:
- Denote $W_t \in \mathbb{R}$ as the market-maker's trading account value at time $t$.
- Denote $I_t \in \mathbb{Z}$ as the market-maker's inventory of shares at time $t$ (assume $I_0=$ 0 ). Note that the inventory can be positive or negative (negative means the marketmaker is short a certain number of shares).
- Denote $S_t \in \mathbb{R}^{+}$as the OB Mid Price at time $t$ (assume a stochastic process for $S_t$ ).
- Denote $P_t^{(b)} \in \mathbb{R}^{+}$as the market-maker's Bid Price at time $t$.
- Denote $N_t^{(b)} \in \mathbb{Z}^{+}$as the market-maker's Bid Shares at time $t$.
- Denote $P_t^{(a)} \in \mathbb{R}^{+}$as the market-maker's Ask Price at time $t$.
- Denote $N_t^{(a)} \in \mathbb{Z}^{+}$as the market-maker's Ask Shares at time $t$.
- We refer to $\delta_t^{(b)}=S_t-P_t^{(b)}$ as the market-maker's Bid Spread (relative to OB Mid).
- We refer to $\delta_t^{(a)}=P_t^{(a)}-S_t$ as the market-maker's Ask Spread (relative to OB Mid).
- We refer to $\delta_t^{(b)}+\delta_t^{(a)}=P_t^{(a)}-P_t^{(b)}$ as the market-maker's Bid-Ask Spread.
- Random variable $X_t^{(b)} \in \mathbb{Z}_{\geq 0}$ refers to the total number of market-maker's Bid Shares that have been transacted against (by MOs or by Sell LOs) up to time $t\left(X_t^{(b)}\right.$ is often referred to as the cumulative "hits" up to time $t$, as in "the market-maker's buy offer has been $h i t^{\prime \prime}$ ).
- Random variable $X_t^{(a)} \in \mathbb{Z}_{\geq 0}$ refers to the total number of market-maker's Ask Shares that have been transacted against (by MOs or by Buy LOs) up to time $t\left(X_t^{(a)}\right.$ is often referred to as the cumulative "lifts" up to time $t$, as in "the market-maker's sell offer has been lifted").

With this notation, we can write the trading account balance equation for all $t=0,1, \ldots, T-1$ as follows:
$$
W_{t+1}=W_t+P_t^{(a)} \cdot\left(X_{t+1}^{(a)}-X_t^{(a)}\right)-P_t^{(b)} \cdot\left(X_{t+1}^{(b)}-X_t^{(b)}\right)
$$

Note that since the inventory $I_0$ at time 0 is equal to 0 , the inventory $I_t$ at time $t$ is given by the equation:
$$
I_t=X_t^{(b)}-X_t^{(a)}
$$

The market-maker's goal is to maximize (for an appropriately shaped concave utility function $U(\cdot))$ the sum of the trading account value at time $T$ and the value of the inventory of shares held at time $T$, i.e., we maximize:
$$
\mathbb{E}\left[U\left(W_T+I_T \cdot S_T\right)\right]
$$