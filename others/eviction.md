Eviction in Kubernetes (k8s) refers to the process by which pods are terminated as a response to certain conditions in a cluster. This process is crucial for maintaining cluster health and stability. Let's break down what eviction is, how it works, and how to perform it:

### What is Eviction in Kubernetes?

1. **Resource Management:** Eviction is primarily a resource management mechanism. It ensures that resources like CPU, memory, and storage are optimally used and not over-committed.

2. **Node Pressure:** When a node in the Kubernetes cluster experiences resource scarcity (like low memory or disk space), it may trigger the eviction process to free up resources.

3. **Quality of Service (QoS):** Kubernetes classifies pods into different QoS classes (Guaranteed, Burstable, BestEffort) based on their resource requests and limits. This classification influences which pods are evicted first.

### How Eviction Works

1. **Resource Monitoring:** Kubernetes constantly monitors resource usage on each node.

2. **Eviction Thresholds:** Administrators can set eviction thresholds. When resource usage exceeds these thresholds, the kubelet (a primary node agent) starts the eviction process.

3. **Pod Selection for Eviction:** Pods with lower QoS classes (like BestEffort) are selected first for eviction. If necessary, it moves up to higher QoS classes.

4. **Graceful Termination:** Evicted pods are given a grace period to shut down gracefully. If they don’t terminate within this period, they are forcibly killed.

### How to Perform Eviction

1. **Manual Eviction:** You can manually evict pods using the `kubectl drain` command. This is often done for maintenance, like before upgrading a node. `kubectl drain <node-name>` safely evicts all pods from the specified node.

2. **Setting Eviction Policies:**
   - Use the `--eviction-hard` or `--eviction-soft` flags in the kubelet configuration to set eviction thresholds based on memory, CPU, disk space, etc.
   - Example: `--eviction-hard=memory.available<500Mi` sets a hard eviction threshold when available memory goes below 500 MiB.

3. **PodDisruptionBudget (PDB):** A PDB limits the number of concurrent disruptions that your application experiences. This can help manage the eviction process by ensuring a minimum number of replicas of a pod are running at all times.

4. **Monitoring and Adjustment:** Regularly monitor the resource usage and eviction events. Adjust the eviction policies and thresholds as necessary based on the behavior observed.

### Best Practices

- **Proper Resource Allocation:** Ensure pods have appropriate resource requests and limits to avoid unnecessary evictions.
- **Monitor Node Health:** Keep an eye on node health and resource utilization.
- **Use PDBs Wisely:** Define PodDisruptionBudgets for applications that need high availability.
- **Graceful Handling in Applications:** Design your applications to handle termination gracefully, especially in stateful applications.

Eviction is a critical aspect of Kubernetes for ensuring resource efficiency and stability. Understanding and correctly configuring eviction policies is key to maintaining a healthy Kubernetes environment.


### Reference

https://kubernetes.io/docs/concepts/scheduling-eviction/node-pressure-eviction/
