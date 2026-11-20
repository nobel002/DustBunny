# Physics
I seem to be kinda getting stuck on the physics


## Basics
Algemene bewegingsvergelijken:

$$
\dot q_k = \frac{\partial H}{\partial p_k}(q_k, p_k,  t)
$$

$$
\dot p_k =  - \frac{\partial H}{\partial q_k} (q_k, p_k, t)
$$



## N-Body
$$
    H(\vec p, \vec q) = \sum p_i ^2/(2 m_i) + \sum_{i, j|i < j} V(q_i, q_j)$$

Nu stelen we volgende potentiaal voor:
(Deze is attractief als K<0)
$
V(q_i, q_j) = \frac{K}{|q_i - q_j|}
$

We gebruiken een potentiaal met een minimum:
$
V(q_i, q_j) = \frac{K}{|q_i - q_j|} + \frac{G}{|q_i - q_j|^n}
$
We gebruiken $n=2$, $K=-40$ en $G=20$ (zie `ParticleSystem.rs`). Het minimum ligt dan op $r_0 = -2G/K = 1$.

We rekenen nu de termen $\frac{\partial H}{\partial p_k}(q_k, p_k,  t)$ en  $\frac{\partial H}{\partial q_k} (q_k, p_k, t)$
uit.


$$
    \frac{\partial H}{\partial p_k} (q_k, p_k, t) = \frac{p_k}{m_k}
$$


$$
    \frac{\partial H}{\partial q_k} = -K \sum_{j \neq k} \frac{q_k - q_j}{|q_k - q_j|^3} - Gn\sum_{j \neq k} \frac{q_k - q_j}{|q_k-q_j|^{n+2}} 
$$

Which is in essence a second order force