# Echo Server

A Rust implementation of an HTTP echo server.  It will echo the HTTP request as the HTTP response.

## Deployment

### Docker

Run the following command to host the echo server on port 3000:

```shell
docker run -p 3000:3000 --rm --name echo-server ghcr.io/codygreen/echo-server:main
```

### Kubernetes

Create the following `echo.yml` file to deploy the echo service via node port on port 30000:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: echo-server-deployment
  labels:
    app: echo-server
spec:
  replicas: 1
  selector:
    matchLabels:
      app: echo-server
  template:
    metadata:
      labels:
        app: echo-server
    spec:
      containers:
      - name: echo-server
        image: ghcr.io/codygreen/echo-server:main
        ports:
        - containerPort: 3000

---
apiVersion: v1
kind: Service
metadata:
  name: echo-server-service
spec:
  type: NodePort
  selector:
    app: echo-server
  ports:
  - port: 3000
    targetPort: 3000
    nodePort: 30000
```

Run the following command to deploy:

```shell
kubectl apply -f echo.yml
```
