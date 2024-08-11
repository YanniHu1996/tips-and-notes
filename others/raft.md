
It communicates with the consensus modules on other
servers to ensure that every log eventually contains the
same requests in the same order,

---

#### List key properties of the algorithm

the key safety property for Raft is the State Machine Safety Property


Election Safety: at most one leader can be elected in a given term. 

Leader Append-Only: a leader never overwrites or deletes entries in its log; it only appends new entries. §5.3

Log Matching: if two logs contain an entry with the same index and term, then the logs are identical in all entries up through the given index. §5.3

Leader Completeness: if a log entry is committed in a
given term, then that entry will be present in the logs
of the leaders for all higher-numbered terms. §5.4

State Machine Safety: if a server has applied a log entry
at a given index to its state machine, no other server
will ever apply a different log entry for the same index.

--- 

Raft ensures that there is at most one
leader in a given term.

---

Different servers may observe the transitions between
terms at different times, and in some situations a server
may not observe an election or even entire terms. Terms
act as a logical clock in Raft, and they allow servers
to detect obsolete information such as stale leaders

--- 

When servers start up, they begin as followers

--- 

To begin an election, a follower increments its current
term and transitions to candidate state. It then votes for
itself and issues RequestVote RPCs in parallel to each of
the other servers in the cluster. A candidate continues in
this state until one of three things happens: 

(a) it wins the election, 

(b) another server establishes itself as leader

(c) a period of time goes by with no winner

---

Each candidate
restarts its randomized election timeout at the start of an
election, and it waits for that timeout to elapse before
starting the next election; this reduces the likelihood of
another split vote in the new election.

--- 

In Raft, the leader handles inconsistencies by forcing
the followers’ logs to duplicate its own. This means that
conflicting entries in follower logs will be overwritten
with entries from the leader’s log

--- 

### Reference 

https://raft.github.io/raft.pdf
