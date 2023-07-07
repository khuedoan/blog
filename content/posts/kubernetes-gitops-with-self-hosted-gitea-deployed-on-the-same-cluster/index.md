---
title: GitOps with ArgoCD and self-hosted Gitea deployed on the same Kubernetes cluster
date: 2022-01-01T09:46:26+07:00
summary: How to solve the chicken and egg problem in GitOps when you self-host your own Git service
tags:
  - kubernetes
  - gitops
draft: true
---

## What is GitOps

GitOps is a pattern of using Git repositories as the source of truth for defining the desired application state.

A usual GitOps setup consist of a continuous delivery tool which pull the desired state from a Git service.
In this article I will use [ArgoCD](https://argo-cd.readthedocs.io) for the continuous delivery tool and [Gitea](https://gitea.io) for the self-hosted Git service, but it should be similar for other tools.

## The problem with self-hosted Git server

Usually it's plug and play if you're using a managed Git service, just point ArgoCD to your GitHub/GitLab repository, but if you self-host your own Git server, you have a chicken and egg problem:
since you have no where to put the desired state to because the Git server is not deployed yet, and if you don't have a desired state for the Git server then ArgoCD doesn't know how to deploy the Git server.

Usually the solution for this is to deploy the Git server separately on a VM, but now you have a special snowflake that needs special treatment, and _usually_ you don't have high availability.

## The solution

TL;DR: use a seed repository from any Git service (hosted on GitHub, Gitlab or just a simple Git server on docker-compose), deploy Gitea with a Kubernetes Job which initialize a GitOps repo from the seed repo, then make ArgoCD automatically switch to the new repository after Gitea is healthy.

### The seed repository


### Deploy Gitea


### Initialize the GitOps repo automatically


### Make ArgoCD switches to the self-hosted Git repo automatically
