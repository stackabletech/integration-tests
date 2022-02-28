# Authorizer Test

Required Operators:
- Zookeeper
- Druid
- RegoRule
- OPA

1. Deploy Zookeeper Cluster
2. Deploy OPA Cluster + RegoRule
3. Deploy Druid Cluster
4. Setup Test Container
5. Run Auth Test:
    - Create two test users: Alice, Eve
    - Run HTTP requests to test if authentication works