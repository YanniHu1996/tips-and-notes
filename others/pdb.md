In Kubernetes (k8s), a PodDisruptionBudget (PDB) is a crucial concept for ensuring the high availability and reliability of applications running in a Kubernetes cluster. It provides a way to limit the number of Pods of a replicated application that are down simultaneously from voluntary disruptions.

### Understanding PodDisruptionBudget

1. **Purpose**: The main goal of a PodDisruptionBudget is to ensure that a specified minimum number of Pods are always running for a given application during voluntary disruptions. Voluntary disruptions can include actions like node maintenance (e.g., kernel upgrades, hardware repairs), scaling down a deployment, or deleting Pods.

2. **Configuration**: A PodDisruptionBudget is defined using labels to select a group of Pods, and specifying the minimum number of Pods that must be available during voluntary disruptions (`minAvailable`) or the maximum number of Pods that can be unavailable (`maxUnavailable`).

### Sample Scenarios

#### Scenario 1: Ensuring Minimum Availability
- **Objective**: Ensure at least 3 Pods of an application are always running.
- **YAML Configuration**:
  ```yaml
  apiVersion: policy/v1
  kind: PodDisruptionBudget
  metadata:
    name: pdb-min-example
  spec:
    minAvailable: 3
    selector:
      matchLabels:
        app: my-application
  ```

#### Scenario 2: Limiting Maximum Unavailability
- **Objective**: Ensure no more than 2 Pods of an application are unavailable during maintenance.
- **YAML Configuration**:
  ```yaml
  apiVersion: policy/v1
  kind: PodDisruptionBudget
  metadata:
    name: pdb-max-example
  spec:
    maxUnavailable: 2
    selector:
      matchLabels:
        app: my-application
  ```

#### Scenario 3: Using with Deployments
- **Objective**: Protect a Deployment with a PDB.
- **Deployment YAML**:
  ```yaml
  apiVersion: apps/v1
  kind: Deployment
  metadata:
    name: my-deployment
  spec:
    replicas: 5
    ...
  ```
- **PDB YAML**:
  ```yaml
  apiVersion: policy/v1
  kind: PodDisruptionBudget
  metadata:
    name: pdb-deployment-example
  spec:
    minAvailable: 4
    selector:
      matchLabels:
        app: my-application
  ```

### Best Practices

- **Use with Replicated Applications**: PDBs are most effective with replicated applications (like Deployments, StatefulSets, etc.).
- **Define Reasonable Budgets**: Setting the PDB values too high can block node maintenance, while setting them too low can compromise application availability.
- **Regular Review**: Regularly review and adjust the PDB settings based on the criticality of the application and the cluster's overall health.

PodDisruptionBudgets are integral in managing application resilience and ensuring that cluster maintenance and scaling operations do not disrupt critical services.


### Reference 

https://kubernetes.io/docs/tasks/run-application/configure-pdb/
