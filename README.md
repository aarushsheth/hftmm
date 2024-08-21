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

The goal is to find an Optimal Policy $\pi^* =\left(\pi_0^*, \pi_1^*, \ldots, \pi_{T-1}^*\right)$, where

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


The following solution is as presented in the Avellaneda-Stoikov paper. We can express the Avellaneda-Stoikov continuous-time formulation as a Hamilton-Jacobi-Bellman (HJB) formulation (note: for reference, the general HJB formulation is covered in Appendix D).

We denote the Optimal Value function as $V^*\left(t, S_t, W_t, I_t\right)$. Note that unlike Section 5.13 in Chapter 5 where we denoted the Optimal Value Function as a time-indexed sequence $V_t^*(\cdot)$, here we make $t$ an explicit functional argument of $V^*$ and each of $S_t, W_t, I_t$ also as separate functional arguments of $V^*$ (instead of the typical approach of making the state, as a tuple, a single functional argument). This is because in the continuous-time setting, we are interested in the time-differential of the Optimal Value Function and we also want to represent the dependency of the Optimal Value Function on each of $S_t, W_t, I_t$ as explicit separate dependencies. Appendix D provides the derivation of the general HJB formulation (Equation (D.1) in Appendix D)—this general HJB Equation specializes here to the following:

$$ 
\max _{\delta_t^{(b)}, \delta_t^{(a)}} \mathbb{E}\left[d V^*\left(t, S_t, W_t, I_t\right)\right]=0 \text { for } t<T
$$

$$V^*\left(T, S_T, W_T, I_T\right)=-e^{-\gamma \cdot\left(W_T+I_T \cdot S_T\right)}$$


An infinitesimal change $d V^*$ to $V^*\left(t, S_t, W_t, I_t\right)$ is comprised of 3 components:
- Due to pure movement in time (dependence of $V^*$ on $t$ ).
- Due to randomness in OB Mid-Price (dependence of $V^*$ on $S_t$ ).
- Due to randomness in hitting/lifting the market-maker's Bid/Ask (dependence of $V^*$ on $\lambda_t^{(b)}$ and $\lambda_t^{(a)}$ ). Note that the probability of being hit in interval from $t$ to $t+$ $d t$ is $\lambda_t^{(b)} \cdot d t$ and probability of being lifted in interval from $t$ to $t+d t$ is $\lambda_t^{(a)} \cdot d t$, upon which the trading account value $W_t$ changes appropriately and the inventory $I_t$ increments/decrements by 1.
With this, we can expand $d V^*\left(t, S_t, W_t, I_t\right)$ and rewrite HJB as:

$$
\max _{\delta_t^{(b)}, \delta_t^{(a)}}\{ \frac{\partial V^*}{\partial t} \cdot d t+\mathbb{E}\left[\sigma \cdot \frac{\partial V^*}{\partial S_t} \cdot d z_t+\frac{\sigma^2}{2} \cdot \frac{\partial^2 V^*}{\partial S_t^2} \cdot\left(d z_t\right)^2\right]
$$

$$+\lambda_t^{(b)} \cdot d t \cdot V^*\left(t, S_t, W_t-S_t+\delta_t^{(b)}, I_t+1\right)$$

$$+\lambda_t^{(a)} \cdot d t \cdot V^*\left(t, S_t, W_t+S_t+\delta_t^{(a)}, I_t-1\right)$$

$$+\left(1-\lambda_t^{(b)} \cdot d t-\lambda_t^{(a)} \cdot d t\right) \cdot V^*\left(t, S_t, W_t, I_t\right)$$

$$\left.-V^*\left(t, S_t, W_t, I_t\right)\right\}=0$$



Next, we want to convert the HJB Equation to a Partial Differential Equation (PDE). We can simplify the above HJB equation with a few observations:
- $\mathbb{E}\left[d z_t\right]=0$.
- $\mathbb{E}\left[\left(d z_t\right)^2\right]=d t$.
- Organize the terms involving $\lambda_t^{(b)}$ and $\lambda_t^{(a)}$ better with some algebra.
- Divide throughout by $d t$.

$$\max _{\delta_t^{(b)}, \delta_t^{(a)}}\{ \frac{\partial V^*}{\partial t}+\frac{\sigma^2}{2} \cdot \frac{\partial^2 V^*}{\partial S_t^2}$$

$$+\lambda_t^{(b)} \cdot\left(V^*\left(t, S_t, W_t-S_t+\delta_t^{(b)}, I_t+1\right)-V^*\left(t, S_t, W_t, I_t\right)\right)$$

$$\left.+\lambda_t^{(a)} \cdot\left(V^*\left(t, S_t, W_t+S_t+\delta_t^{(a)}, I_t-1\right)-V^*\left(t, S_t, W_t, I_t\right)\right)\right\}=0$$

Next, note that $\lambda_t^{(b)}=f^{(b)}\left(\delta_t^{(b)}\right)$ and $\lambda_t^{(a)}=f^{(a)}\left(\delta_t^{(a)}\right)$, and apply the max only on the relevant terms:

$$\frac{\partial V^*}{\partial t}+\frac{\sigma^2}{2} \cdot \frac{\partial^2 V^*}{\partial S_t^2}$$

$$+\max _{\delta_t^{(b)}}\left\{f^{(b)}\left(\delta_t^{(b)}\right) \cdot\left(V^*\left(t, S_t, W_t-S_t+\delta_t^{(b)}, I_t+1\right)-V^*\left(t, S_t, W_t, I_t\right)\right)\right\}$$

$$+\max _{\delta_t^{(a)}}\left\{f^{(a)}\left(\delta_t^{(a)}\right) \cdot\left(V^*\left(t, S_t, W_t+S_t+\delta_t^{(a)}, I_t-1\right)-V^*\\left(t, S_t, W_t, I_t\right)\right)\}=0$$


This combines with the boundary condition:

$$
V^*\left(T, S_T, W_T, I_T\right)=-e^{-\gamma \cdot\left(W_T+I_T \cdot S_T\right)}
$$

Next, we make an educated guess for the functional form of $V^*\left(t, S_t, W_t, I_t\right)$ :

$$
V^*\left(t, S_t, W_t, I_t\right)=-e^{-\gamma \cdot\left(W_t+\theta\left(t, S_t, I_t\right)\right)}
$$

to reduce the problem to a Partial Differential Equation (PDE) in terms of $\theta\left(t, S_t, I_t\right)$. Substituting this guessed functional form into the above PDE for $V^*\left(t, S_t, W_t, I_t\right)$ gives:

$$\frac{\partial \theta}{\partial t}+\frac{\sigma^2}{2} \cdot\left(\frac{\partial^2 \theta}{\partial S_t^2}-\gamma \cdot\left(\frac{\partial \theta}{\partial S_t}\right)^2\right)$$

$$+\max _{\delta_t^{(b)}}\left\{\frac{f^{(b)}\left(\delta_t^{(b)}\right)}{\gamma} \cdot\left(1-e^{-\gamma \cdot\left(\delta_t^{(b)}-S_t+\theta\left(t, S_t, I_t+1\right)-\theta\left(t, S_t, I_t\right)\right)}\right)\right\}$$

$$+\max _{\delta_t^{(a)}}\left\{\frac{f^{(a)}\left(\delta_t^{(a)}\right)}{\gamma} \cdot\left(1-e^{-\gamma \cdot\left(\delta_t^{(a)}+S_t+\theta\left(t, S_t, I_t-1\right)-\theta\left(t, S_t, I_t\right)\right)}\right)\right\}=0$$


The boundary condition is:

$$
\theta\left(T, S_T, I_T\right)=I_T \cdot S_T
$$

It turns out that $\theta\left(t, S_t, I_t+1\right)-\theta\left(t, S_t, I_t\right)$ and $\theta\left(t, S_t, I_t\right)-\theta\left(t, S_t, I_t-1\right)$ are equal to financially meaningful quantities known as Indifference Bid and Ask Prices.

Indifference Bid Price $Q^{(b)}\left(t, S_t, I_t\right)$ is defined as follows:

$$
V^*\left(t, S_t, W_t-Q^{(b)}\left(t, S_t, I_t\right), I_t+1\right)=V^*\left(t, S_t, W_t, I_t\right)
$$

$Q^{(b)}\left(t, S_t, I_t\right)$ is the price to buy a single share with a guarantee of immediate purchase that results in the Optimum Expected Utility staying unchanged.

Likewise, Indifference Ask Price $Q^{(a)}\left(t, S_t, I_t\right)$ is defined as follows:

$$
V^*\left(t, S_t, W_t+Q^{(a)}\left(t, S_t, I_t\right), I_t-1\right)=V^*\left(t, S_t, W_t, I_t\right)
$$

$Q^{(a)}\left(t, S_t, I_t\right)$ is the price to sell a single share with a guarantee of immediate sale that results in the Optimum Expected Utility staying unchanged.

For convenience, we abbreviate $Q^{(b)}\left(t, S_t, I_t\right)$ as $Q_t^{(b)}$ and $Q^{(a)}\left(t, S_t, I_t\right)$ as $Q_t^{(a)}$. Next, we express $V^*\left(t, S_t, W_t-Q_t^{(b)}, I_t+1\right)=V^*\left(t, S_t, W_t, I_t\right)$ in terms of $\theta$ :

$$-e^{-\gamma \cdot\left(W_t-Q_t^{(b)}+\theta\left(t, S_t, I_t+1\right)\right)}=-e^{-\gamma \cdot\left(W_t+\theta\left(t, S_t, I_t\right)\right)}$$

$$\Rightarrow Q_t^{(b)}=\theta\left(t, S_t, I_t+1\right)-\theta\left(t, S_t, I_t\right)$$


Likewise for $Q_t^{(a)}$, we get:
$$
Q_t^{(a)}=\theta\left(t, S_t, I_t\right)-\theta\left(t, S_t, I_t-1\right)
$$

Using Equations (10.14) and (10.15), bring $Q_t^{(b)}$ and $Q_t^{(a)}$ in the PDE for $\theta$ :

$$\frac{\partial \theta}{\partial t}+\frac{\sigma^2}{2} \cdot\left(\frac{\partial^2 \theta}{\partial S_t^2}-\gamma \cdot\left(\frac{\partial \theta}{\partial S_t}\right)^2\right)+\max _{\delta_t^{(b)}} g\left(\delta_t^{(b)}\right)+\max _{\delta_t^{(a)}} h\left(\delta_t^{(b)}\right)=0$$

$$\text {where } g\left(\delta_t^{(b)}\right)=\frac{f^{(b)}\left(\delta_t^{(b)}\right)}{\gamma} \cdot\left(1-e^{-\gamma \cdot\left(\delta_t^{(b)}-S_t+Q_t^{(b)}\right)}\right)$$

$$\text {and } h\left(\delta_t^{(a)}\right)=\frac{f^{(a)}\left(\delta_t^{(a)}\right)}{\gamma} \cdot\left(1-e^{\left.-\gamma \cdot \delta_t^{(a)}+S_t-Q_t^{(a)}\right)}\right)$$

To maximize $g\left(\delta_t^{(b)}\right)$, differentiate $g$ with respect to $\delta_t^{(b)}$ and set to 0 :

$$e^{-\gamma \cdot\left(\delta_t^{(b) *}-S_t+Q_t^{(b)}\right)} \cdot\left(\gamma \cdot f^{(b)}\left(\delta_t^{(b)^*}\right)-\frac{\partial f^{(b)}}{\partial \delta_t^{(b)}}\left(\delta_t^{(b)^*}\right)\right)+\frac{\partial f^{(b)}}{\partial \delta_t^{(b)}}\left(\delta_t^{(b)^*}\right)=0$$

$$\Rightarrow \delta_t^{(b)^*}=S_t-P_t^{(b)^*}=S_t-Q_t^{(b)}+\frac{1}{\gamma} \cdot \log \left(1-\gamma \cdot \frac{f^{(b)}\left(\delta_t^{(b)^*}\right)}{\frac{\partial f^{(b)}}{\partial \delta_t^{(b)}}\left(\delta_t^{(b)^*}\right)}\right)$$

To maximize $h\left(\delta_t^{(a)}\right)$, differentiate $h$ with respect to $\delta_t^{(a)}$ and set to 0 :

$$e^{-\gamma \cdot\left(\delta_t^{(a)^*}+S_t-Q_t^{(a)}\right)} \cdot\left(\gamma \cdot f^{(a)}\left(\delta_t^{(a)^*}\right)-\frac{\partial f^{(a)}}{\partial \delta_t^{(a)}}\left(\delta_t^{(a)^*}\right)\right)+\frac{\partial f^{(a)}}{\partial \delta_t^{(a)}}\left(\delta_t^{(a)^*}\right)=0$$

$$\Rightarrow \delta_t^{(a)^*}=P_t^{(a)^*}-S_t=Q_t^{(a)}-S_t+\frac{1}{\gamma} \cdot \log \left(1-\gamma \cdot \frac{f^{(a)}\left(\delta_t^{(a)^*}\right)}{\frac{\partial f^{(a)}}{\partial \delta_t^{(a)}}\left(\delta_t^{(a)^*}\right)}\right)$$


Equations (10.16) and (10.17) are implicit equations for $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$, respectively. Now let us write the PDE in terms of the Optimal Bid and Ask Spreads:

$$\frac{\partial \theta}{\partial t}+\frac{\sigma^2}{2} \cdot\left(\frac{\partial^2 \theta}{\partial S_t^2}-\gamma \cdot\left(\frac{\partial \theta}{\partial S_t}\right)^2\right)$$

$$+\frac{f^{(b)}\left(\delta_t^{(b)^*}\right)}{\gamma} \cdot\left(1-e^{-\gamma \cdot\left(\delta_t^{(b) *}-S_t+\theta\left(t, S_t, I_t+1\right)-\theta\left(t, S_t, I_t\right)\right)}\right)$$

$$+\frac{f^{(a)}\left(\delta_t^{(a)^*}\right)}{\gamma} \cdot\left(1-e^{-\gamma \cdot\left(\delta_t^{(a)^*}+S_t+\theta\left(t, S_t, I_t-1\right)-\theta\left(t, S_t, I_t\right)\right)}\right)=0$$

with boundary condition: $\theta\left(T, S_T, I_T\right)=I_T \cdot S_T$
How do we go about solving this? Here are the steps:

- Firstly, we solve $\operatorname{PDE}$ (10.18) for $\theta$ in terms of $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$. In general, this would be a numerical PDE solution.
- Using Equations (10.14) and (10.15), and using the above-obtained $\theta$ in terms of $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$, we get $Q_t^{(b)}$ and $Q_t^{(a)}$ in terms of $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$.
- Then we substitute the above-obtained $Q_t^{(b)}$ and $Q_t^{(a)}$ (in terms of $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$ ) in Equations (10.16) and (10.17).
- Finally, we solve the implicit equations for $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$ (in general, numerically).

This completes the (numerical) solution to the Avellaneda-Stoikov continuous-time formulation for the Optimal Market-Making problem. Having been through all the heavy equations above, let's now spend some time on building intuition.

Define the Indifference Mid Price $Q_t^{(m)}=\frac{Q_t^{(b)}+Q_t^{(a)}}{2}$. To develop intuition for Indifference Prices, consider a simple case where the market-maker doesn't supply any bids or asks after time $t$. This means the trading account value $W_T$ at time $T$ must be the same as the trading account value at time $t$ and the inventory $I_T$ at time $T$ must be the same as the inventory $I_t$ at time $t$. This implies:

$$
V^*\left(t, S_t, W_t, I_t\right)=\mathbb{E}\left[-e^{-\gamma \cdot\left(W_t+I_t \cdot S_T\right)}\right]
$$

The process $d S_t=\sigma \cdot d z_t$ implies that $S_T \sim \mathcal{N}\left(S_t, \sigma^2 \cdot(T-t)\right)$, and hence:

$$
V^*\left(t, S_t, W_t, I_t\right)=-e^{-\gamma \cdot\left(W_t+I_t \cdot S_t-\frac{\gamma \cdot I_t^2 \cdot \sigma^2 \cdot(T-t)}{2}\right)}
$$

Hence,

$$
V^*\left(t, S_t, W_t-Q_t^{(b)}, I_t+1\right)=-e^{-\gamma \cdot\left(W_t-Q_t^{(b)}+\left(I_t+1\right) \cdot S_t-\frac{\gamma \cdot\left(I_t+1\right)^2 \cdot \sigma^2 \cdot(T-t)}{2}\right)}
$$

But from Equation (10.12), we know that:

$$
V^*\left(t, S_t, W_t, I_t\right)=V^*\left(t, S_t, W_t-Q_t^{(b)}, I_t+1\right)
$$

Therefore,

$$
-e^{-\gamma \cdot\left(W_t+I_t \cdot S_t-\frac{\gamma \cdot I_t^2 \cdot \sigma^2 \cdot(T-t)}{2}\right)}=-e^{-\gamma \cdot\left(W_t-Q_t^{(b)}+\left(I_t+1\right) \cdot S_t-\frac{\gamma \cdot\left(I_t+1\right)^2 \cdot \sigma^2 \cdot(T-t)}{2}\right)}
$$

This implies:

$$
Q_t^{(b)}=S_t-\left(2 I_t+1\right) \cdot \frac{\gamma \cdot \sigma^2 \cdot(T-t)}{2}
$$

Likewise, we can derive:

$$
Q_t^{(a)}=S_t-\left(2 I_t-1\right) \cdot \frac{\gamma \cdot \sigma^2 \cdot(T-t)}{2}
$$

The formulas for the Indifference Mid Price and the Indifference Bid-Ask Price Spread are as follows:

$$Q_t^{(m)}=S_t-I_t \cdot \gamma \cdot \sigma^2 \cdot(T-t)$$

$$Q_t^{(a)}-Q_t^{(b)}=\gamma \cdot \sigma^2 \cdot(T-t)$$

These results for the simple case of no-market-making-after-time-t serve as approximations for our problem of optimal market-making. Think of $Q_t^{(m)}$ as a pseudo mid price for the market-maker, an adjustment to the OB mid price $S_t$ that takes into account the magnitude and sign of $I_t$. If the market-maker is long inventory $\left(I_t>0\right)$, then $Q_t^{(m)}<S_t$, which makes intuitive sense since the market-maker is interested in reducing her risk of inventory buildup and so, would be be more inclined to sell than buy, leading her to show bid and ask prices whose average is lower than the OB mid price $S_t$. Likewise, if the marketmaker is short inventory $\left(I_t<0\right)$, then $Q_t^{(m)}>S_t$ indicating inclination to buy rather than sell.

Armed with this intuition, we come back to optimal market-making, observing from Equations (10.16) and (10.17):

$$
P_t^{(b)^*}<Q_t^{(b)}<Q_t^{(m)}<Q_t^{(a)}<P_t^{(a)^*}
$$

Visualize this ascending sequence of prices $\left[P_t^{(b)^*}, Q_t^{(b)}, Q_t^{(m)}, Q_t^{(a)}, P_t^{(a)^*}\right]$ as jointly sliding up/down (relative to OB mid price $S_t$ ) as a function of the inventory $I_t^{\prime}$ 's magnitude and sign, and perceive $P_t^{(b)^*}, P_t^{(a)^*}$ in terms of their spreads to the pseudo mid price $Q_t^{(m)}$ :

$$Q_t^{(b)}-P_t^{(m)^*}=\frac{Q_t^{(b)}+Q_t^{(a)}}{2}+\frac{1}{\gamma} \cdot \log \left(1-\gamma \cdot \frac{f^{(b)}\left(\delta_t^{(b)^*}\right)}{\frac{\partial f^{(b)}}{\partial \delta_t^{(b)}}\left(\delta_t^{(b)^*}\right)}\right)$$

$$P_t^{(a)^*}-Q_t^{(m)}=\frac{Q_t^{(b)}+Q_t^{(a)}}{2}+\frac{1}{\gamma} \cdot \log \left(1-\gamma \cdot \ \frac{f^{(a)}\left(\delta_t^{(a)^*}\right)}{\frac{\partial f^{(a)}}{\partial \delta_t^{(a)}}\left(\delta_t^{(a)^*}\right)}\right)$$



The PDE (10.18) we derived above for $\theta$ and the associated implicit Equations (10.16) and (10.17) for $\delta_t^{(b)^*}, \delta_t^{(a)^*}$ are messy. So we make some assumptions, simplify, and derive analytical approximations(as presented in the Avellaneda-Stoikov paper). We start by assuming a fairly standard functional form for $f^{(b)}$ and $f^{(a)}$ :

$$
f^{(b)}(\delta)=f^{(a)}(\delta)=c \cdot e^{-k \cdot \delta}
$$


This reduces Equations (10.16) and (10.17) to:

$$\delta_t^{(b)^*}=S_t-Q_t^{(b)}+\frac{1}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)$$

$$\delta_t^{(a)^*}=Q_t^{(a)}-S_t+\frac{1}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)$$


which means $P_t^{(b)^*}$ and $P_t^{(a)^*}$ are equidistant from $Q_t^{(m)}$. Substituting these simplified $\delta_t^{(b)^*}, \delta_t^{(a)^*}$ in Equation (10.18) reduces the PDE to:

$$
\frac{\partial \theta}{\partial t}+\frac{\sigma^2}{2} \cdot\left(\frac{\partial^2 \theta}{\partial S_t^2}-\gamma \cdot\left(\frac{\partial \theta}{\partial S_t}\right)^2\right)+\frac{c}{k+\gamma} \cdot\left(e^{-k \cdot \delta_t^{(b)^*}}+e^{-k \cdot \delta_t^{(a)^*}}\right)=0
$$

$$
\text { with boundary condition } \theta\left(T, S_T, I_T\right)=I_T \cdot S_T
$$

Note that this PDE (10.21) involves $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$. However, Equations (10.19), (10.20), (10.14) and (10.15) enable expressing $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$ in terms of $\theta\left(t, S_t, I_t-1\right), \theta\left(t, S_t, I_t\right)$ and $\theta\left(t, S_t, I_t+1\right)$. This gives us a PDE just in terms of $\theta$. Solving that PDE for $\theta$ would give us not only $V^*\left(t, S_t, W_t, I_t\right)$ but also $\delta_t^{(b)^*}$ and $\delta_t^{(a)^*}$ (using Equations (10.19), (10.20), (10.14) and (10.15)). To solve the PDE, we need to make a couple of approximations.

First, we make a linear approximation for $e^{-k \cdot \delta_t^{(b)^*}}$ and $e^{-k \cdot \delta_t^{(a)^*}}$ in PDE (10.21) as follows:

$$
\frac{\partial \theta}{\partial t}+\frac{\sigma^2}{2} \cdot\left(\frac{\partial^2 \theta}{\partial S_t^2}-\gamma \cdot\left(\frac{\partial \theta}{\partial S_t}\right)^2\right)+\frac{c}{k+\gamma} \cdot\left(1-k \cdot \delta_t^{(b)^*}+1-k \cdot \delta_t^{(a)^*}\right)=0
$$

Combining the Equations (10.19), (10.20), (10.14) and (10.15) gives us:

$$
\delta_t^{(b)^*}+\delta_t^{(a)^*}=\frac{2}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)+2 \theta\left(t, S_t, I_t\right)-\theta\left(t, S_t, I_t+1\right)-\theta\left(t, S_t, I_t-1\right)
$$

With this expression for $\delta_t^{(b)^*}+\delta_t^{(a)^*}, \operatorname{PDE}(10.22)$ takes the form:

$$
\begin{aligned}
\frac{\partial \theta}{\partial t}+\frac{\sigma^2}{2} \cdot & \left(\frac{\partial^2 \theta}{\partial S_t^2}-\gamma \cdot\left(\frac{\partial \theta}{\partial S_t}\right)^2\right)+\frac{c}{k+\gamma} \cdot\left(2-\frac{2 k}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)\right. \\
& \left.-k \cdot\left(2 \theta\left(t, S_t, I_t\right)-\theta\left(t, S_t, I_t+1\right)-\theta\left(t, S_t, I_t-1\right)\right)\right)=0
\end{aligned}
$$

To solve PDE (10.23), we consider the following asymptotic expansion of $\theta$ in $I_t$ :

$$
\theta\left(t, S_t, I_t\right)=\sum_{n=0}^{\infty} \frac{I_t^n}{n!} \cdot \theta^{(n)}\left(t, S_t\right)
$$

So we need to determine the functions $\theta^{(n)}\left(t, S_t\right)$ for all $n=0,1,2, \ldots$
For tractability, we approximate this expansion to the first 3 terms:

$$
\theta\left(t, S_t, I_t\right) \approx \theta^{(0)}\left(t, S_t\right)+I_t \cdot \theta^{(1)}\left(t, S_t\right)+\frac{I_t^2}{2} \cdot \theta^{(2)}\left(t, S_t\right)
$$

We note that the Optimal Value Function $V^*$ can depend on $S_t$ only through the current Value of the Inventory (i.e., through $I_t \cdot S_t$ ), i.e., it cannot depend on $S_t$ in any other way. This
means $V^*\left(t, S_t, W_t, 0\right)=-e^{-\gamma\left(W_t+\theta^{(0)}\left(t, S_t\right)\right)}$ is independent of $S_t$. This means $\theta^{(0)}\left(t, S_t\right)$ is independent of $S_t$. So, we can write it as simply $\theta^{(0)}(t)$, meaning $\frac{\partial \theta^{(0)}}{\partial S_t}$ and $\frac{\partial^2 \theta^{(0)}}{\partial S_t^2}$ are equal to 0 . Therefore, we can write the approximate expansion for $\theta\left(t, S_t, I_t\right)$ as:

$$
\theta\left(t, S_t, I_t\right)=\theta^{(0)}(t)+I_t \cdot \theta^{(1)}\left(t, S_t\right)+\frac{I_t^2}{2} \cdot \theta^{(2)}\left(t, S_t\right)
$$

Substituting this approximation Equation (10.24) for $\theta\left(t, S_t, I_t\right)$ in $\operatorname{PDE}$ (10.23), we get:

$$
\begin{aligned}
& \frac{\partial \theta^{(0)}}{\partial t}+I_t \cdot \frac{\partial \theta^{(1)}}{\partial t}+\frac{I_t^2}{2} \cdot \frac{\partial \theta^{(2)}}{\partial t}+\frac{\sigma^2}{2} \cdot\left(I_t \cdot \frac{\partial^2 \theta^{(1)}}{\partial S_t^2}+\frac{I_t^2}{2} \cdot \frac{\partial^2 \theta^{(2)}}{\partial S_t^2}\right) \\
& -\frac{\gamma \sigma^2}{2} \cdot\left(I_t \cdot \frac{\partial \theta^{(1)}}{\partial S_t}+\frac{I_t^2}{2} \cdot \frac{\partial \theta^{(2)}}{\partial S_t}\right)^2+\frac{c}{k+\gamma} \cdot\left(2-\frac{2 k}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)+k \cdot \theta^{(2)}\right)=0
\end{aligned}
$$

with boundary condition:

$$
\theta^{(0)}(T)+I_T \cdot \theta^{(1)}\left(T, S_T\right)+\frac{I_T^2}{2} \cdot \theta^{(2)}\left(T, S_T\right)=I_T \cdot S_T
$$

Now we separately collect terms involving specific powers of $I_t$, each yielding a separate PDE:
- Terms devoid of $I_t$ (i.e., $I_t^0$ )
- Terms involving $I_t$ (i.e., $I_t^1$ )
- Terms involving $I_t^2$

We start by collecting terms involving $I_t$ :

$$
\frac{\partial \theta^{(1)}}{\partial t}+\frac{\sigma^2}{2} \cdot \frac{\partial^2 \theta^{(1)}}{\partial S_t^2}=0 \text { with boundary condition } \theta^{(1)}\left(T, S_T\right)=S_T
$$

The solution to this PDE is:

$$
\theta^{(1)}\left(t, S_t\right)=S_t
$$

Next, we collect terms involving $I_t^2$ :

$$
\frac{\partial \theta^{(2)}}{\partial t}+\frac{\sigma^2}{2} \cdot \frac{\partial^2 \theta^{(2)}}{\partial S_t^2}-\gamma \cdot \sigma^2 \cdot\left(\frac{\partial \theta^{(1)}}{\partial S_t}\right)^2=0 \text { with boundary condition } \theta^{(2)}\left(T, S_T\right)=0
$$

Noting that $\theta^{(1)}\left(t, S_t\right)=S_t$, we solve this PDE as:

$$
\theta^{(2)}\left(t, S_t\right)=-\gamma \cdot \sigma^2 \cdot(T-t)
$$

Finally, we collect the terms devoid of $I_t$

$$
\frac{\partial \theta^{(0)}}{\partial t}+\frac{c}{k+\gamma} \cdot\left(2-\frac{2 k}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)+k \cdot \theta^{(2)}\right)=0 \text { with boundary } \theta^{(0)}(T)=0
$$

Noting that $\theta^{(2)}\left(t, S_t\right)=-\gamma \sigma^2 \cdot(T-t)$, we solve as:

$$
\theta^{(0)}(t)=\frac{c}{k+\gamma} \cdot\left(\left(2-\frac{2 k}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)\right) \cdot(T-t)-\frac{k \gamma \sigma^2}{2} \cdot(T-t)^2\right)
$$


This completes the PDE solution for $\theta\left(t, S_t, I_t\right)$ and hence, for $V^*\left(t, S_t, W_t, I_t\right)$. Lastly, we derive formulas for $Q_t^{(b)}, Q_t^{(a)}, Q_t^{(m)}, \delta_t^{(b)^*}, \delta_t^{(a)^*}$.

Using Equations (10.14) and (10.15), we get:

$$
\begin{aligned}
& Q_t^{(b)}=\theta^{(1)}\left(t, S_t\right)+\left(2 I_t+1\right) \cdot \theta^{(2)}\left(t, S_t\right)=S_t-\left(2 I_t+1\right) \cdot \frac{\gamma \cdot \sigma^2 \cdot(T-t)}{2} \\
& Q_t^{(a)}=\theta^{(1)}\left(t, S_t\right)+\left(2 I_t-1\right) \cdot \theta^{(2)}\left(t, S_t\right)=S_t-\left(2 I_t-1\right) \cdot \frac{\gamma \cdot \sigma^2 \cdot(T-t)}{2}
\end{aligned}
$$

Using equations (10.19) and (10.20), we get:

$$
\begin{aligned}
\delta_t^{(b)^*} & =\frac{\left(2 I_t+1\right) \cdot \gamma \cdot \sigma^2 \cdot(T-t)}{2}+\frac{1}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right) \\
\delta_t^{(a)^*} & =\frac{\left(1-2 I_t\right) \cdot \gamma \cdot \sigma^2 \cdot(T-t)}{2}+\frac{1}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)
\end{aligned}
$$

Optimal Bid-Ask Spread $\delta_t^{(b)^*}+\delta_t^{(a)^*}=\gamma \cdot \sigma^2 \cdot(T-t)+\frac{2}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)$

Optimal Pseudo-Mid $Q_t^{(m)}=\frac{Q_t^{(b)}+Q_t^{(a)}}{2}=\frac{P_t^{(b)^*}+P_t^{(a)^*}}{2}=S_t-I_t \cdot \gamma \cdot \sigma^2 \cdot(T-t)$


Now let's get back to developing intuition. Think of $Q_t^{(m)}$ as inventory-risk-adjusted midprice (adjustment to $S_t$ ). If the market-maker is long inventory $\left(I_t>0\right), Q_t^{(m)}<S_t$ indicating inclination to sell rather than buy, and if market-maker is short inventory, $Q_t^{(m)}>S_t$ indicating inclination to buy rather than sell. Think of the interval $\left[P_t^{(b)^*}, P_t^{(a)^*}\right]$ as being around the pseudo mid-price $Q_t^{(m)}$ (rather than thinking of it as being around the OB mid-price $\left.S_t\right)$. The interval $\left[P_t^{(b)^*}, P_t^{(a)^*}\right]$ moves up/down in tandem with $Q_t^{(m)}$ moving up/down (as a function of inventory $I_t$ ). Note from Equation (10.33) that the Optimal Bid-Ask Spread $P_t^{(a)^*}-P_t^{(b)^*}$ is independent of inventory $I_t$.

A useful view is:

$$
P_t^{(b)^*}<Q_t^{(b)}<Q_t^{(m)}<Q_t^{(a)}<P_t^{(a)^*}
$$

with the spreads as follows:

Outer Spreads $P_t^{(a)^*}-Q_t^{(a)}=Q_t^{(b)}-P_t^{(b)^*}=\frac{1}{\gamma} \cdot \log \left(1+\frac{\gamma}{k}\right)$
Inner Spreads $Q_t^{(a)}-Q_t^{(m)}=Q_t^{(m)}-Q_t^{(b)}=\frac{\gamma \cdot \sigma^2 \cdot(T-t)}{2}$

This completes the analytical approximation to the solution of the Avellaneda-Stoikov continuous-time formulation of the Optimal Market-Making problem.

