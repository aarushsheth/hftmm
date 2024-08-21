# High Frequency Trading in a Limit Order Book

Assume there are a finite number of time steps indexed by $t=0,1, \ldots, T$. Assume the market-maker always shows a bid price and ask price (at each time $t$ ) along with the associated bid shares and ask shares on the OB. Also assume, for ease of exposition, that the market-maker can add or remove bid/ask shares from the OB costlessly. We use the following notation:
- Denote $W_t \in \mathbb{R}$ as the market-maker's trading account value at time $t$.
- Denote $I_t \in \mathbb{Z}$ as the market-maker's inventory of shares at time $t$ (assume $I_0=$ 0 ). Note that the inventory can be positive or negative (negative means the marketmaker is short a certain number of shares).
- Denote $S_t \in \mathbb{R}^{+}$ as the OB Mid Price at time $t$ (assume a stochastic process for $S_t$ )
- Denote $P_t^{(b)} \in \mathbb{R}^{+}$ as the market-maker's Bid Price at time $t$.
- Denote $N_t^{(b)} \in \mathbb{Z}^{+}$ as the market-maker's Bid Shares at time $t$.
- Denote $P_t^{(a)} \in \mathbb{R}^{+}$ as the market-maker's Ask Price at time $t$.
- Denote $N_t^{(a)} \in \mathbb{Z}^{+}$ as the market-maker's Ask Shares at time $t$.
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



As we alluded to earlier, this problem can be cast as a discrete-time finite-horizon Markov Decision Process (with discount factor $\gamma=1$ ). Following the usual notation for discrete-time finite-horizon MDPs, the order of activity for the MDP at each time step $t=0,1, \ldots, T-1$ is as follows:

- Observe State $\left(S_t, W_t, I_t\right) \in \mathcal{S}_t$.

- Perform Action $\left(P_t^{(b)}, N_t^{(b)}, P_t^{(a)}, N_t^{(a)}\right) \in \mathcal{A}_t$.
- Random number of bid shares hit at time step $t$ (this is equal to $X_{t+1}^{(b)}-X_t^{(b)}$ ).
- Random number of ask shares lifted at time step $t$ (this is equal to $X_{t+1}^{(a)}-X_t^{(a)}$ ),
- Update of $W_t$ to $W_{t+1}$.
- Update of $I_t$ to $I_{t+1}$.
- Stochastic evolution of $S_t$ to $S_{t+1}$.
- Receive Reward $R_{t+1}$, where

$$
R_{t+1}:= \begin{cases}0 & \text { for } 1 \leq t+1 \leq T-1 \\ U\left(W_T+I_T \cdot S_T\right) & \text { for } t+1=T\end{cases}
$$

The goal is to find an Optimal Policy $\pi^*=\left(\pi_0^*, \pi_1^*, \ldots, \pi_{T-1}^*\right)$, where

$$
\pi_t^*\left(\left(S_t, W_t, I_t\right)\right)=\left(P_t^{(b)}, N_t^{(b)}, P_t^{(a)}, N_t^{(a)}\right)
$$

that maximizes:

$$
\mathbb{E}\left[\sum^T R_t\right]=\mathbb{E}\left[R_T\right]=\mathbb{E}\left[U\left(W_T+I_T \cdot S_T\right)\right]
$$

A landmark paper by Avellaneda and Stoikov (Avellaneda and Stoikov 2008) formulated this optimal market-making problem in its continuous-time version. Their formulation is conducive to analytical tractability and they came up with a simple, clean and intuitive solution. In this subsection, we go over their formulation and in the next subsection, we show the derivation of their solution. We adapt our discrete-time notation above to their continuous-time setting.
$\left[\left(X_t^{(b)} \mid 0 \leq t<T\right]\right.$ and $\left[X_t^{(a)} \mid 0 \leq t<T\right]$ are assumed to be continuous-time Poisson processes with the hit rate per unit of time and the lift rate per unit of time denoted as $\lambda_t^{(b)}$ and $\lambda_t^{(a)}$, respectively. Hence, we can write the following:

$$d X_t^{(b)} \sim {Poisson}\left(\lambda_t^{(b)} \cdot d t\right) $$

$$ d X_t^{(a)} \sim {Poisson}\left(\lambda_t^{(a)} \cdot d t\right) $$

$$ \lambda_t^{(b)}=f^{(b)}\left(\delta_t^{(b)}\right) $$

$$ \lambda_t^{(a)}=f^{(a)}\left(\delta_t^{(a)}\right) $$

for decreasing functions $f^{(b)}(\cdot)$ and $f^{(a)}(\cdot)$.

$$ d W_t=P_t^{(a)} \cdot d X_t^{(a)}-P_t^{(b)} \cdot d X_t^{(b)} $$

$$I_t=X_t^{(b)}-X_t^{(a)}\left(\text { note: } I_0=0\right) $$

Since infinitesimal Poisson random variables $d X_t^{(b)}$ (shares hit in time interval from $t$ to $t+d t$ ) and $d X_t^{(a)}$ (shares lifted in time interval from $t$ to $t+d t$ ) are Bernoulli random variables (shares hit/lifted within time interval of duration $d t$ will be 0 or 1 ), $N_t^{(b)}$ and $N_t^{(a)}$ (number of shares in the submitted LOs for the infinitesimal time interval from $t$ to $t+d t$ ) can be assumed to be 1 .

This simplifies the Action at time $t$ to be just the pair:

$$
\left(\delta_t^{(b)}, \delta_t^{(a)}\right)
$$

OB Mid Price Dynamics is assumed to be scaled Brownian motion:

$$
d S_t=\sigma \cdot d z_t
$$

for some $\sigma \in \mathbb{R}^{+}$.
The Utility function is assumed to be: $U(x)=-e^{-\gamma x}$ where $\gamma>0$ is the risk-aversion parameter (this Utility function is essentially the CARA Utility function devoid of associated constants).


