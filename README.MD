# Intro

This is a sample code from my FOSS talk at [DIT University](https://talks.dhananjaay.dev/foss-ddh-chapter-20-april-2024-load-balancer). The talk was about "Deep Dive Into Load Balancer" and this code is a simple implementation of a load balancer in Rust 🦀.

## Folder Structure

root folder contains the code for base server which would act as proxy , they are written using nodejs and expressjs. The `server` folder contains the code for the backend servers which would be load balanced. The `load_balancer` folder contains the code for the load balancer written in Rust.

## Strategy

The load balancer uses a simple round-robin strategy to distribute the incoming requests to the backend servers.

## Servers Configuration

The base server is running on port 3000 and the backend servers are running on port 3001, 3002, 3003, 3004, 3005.

## How to run

This install the dependencies

```bash
npm install
```

This will start base server and install dependencies for load balancer and start after building it

```bash
npm start
```

## How to test

You can test the load balancer by hitting the base server endpoint `http://localhost:3000` in your browser or using curl.

```bash
curl http://localhost:3000
```

As you hit the endpoint multiple times, you will see the response from different backend servers.
