# rust-app-template

![Version: 0.0.0](https://img.shields.io/badge/Version-0.0.0-informational?style=flat-square) ![Type: application](https://img.shields.io/badge/Type-application-informational?style=flat-square) ![AppVersion: 0.0.0](https://img.shields.io/badge/AppVersion-0.0.0-informational?style=flat-square)

Rust application template Helm chart

## TL;DR

```console
helm install my-release oci://ghcr.io/azavalon/charts/rust-app-template --version 0.0.0
```

## Introduction

This chart deploys [rust-app-template](https://github.com/AzAvalon/rust-app-template)
on a [Kubernetes](https://kubernetes.io) cluster with the [Helm](https://helm.sh)
package manager.

## Prerequisites

- Kubernetes 1.29+
- Helm 3.9.0+ (OCI registries)
- The Gateway API CRDs, only with `ingress.enabled`

## Installing the Chart

Install the chart with the release name `my-release` in the namespace `my-namespace`:

```console
helm install my-release oci://ghcr.io/azavalon/charts/rust-app-template \
  --version 0.0.0 \
  --namespace my-namespace --create-namespace
```

The [Parameters](#parameters) section lists what can be set. Give them with
`--set key=value` or in a values file:

```console
helm install my-release oci://ghcr.io/azavalon/charts/rust-app-template -f my-values.yaml
```

> **Tip**: List all releases using `helm list -aA`

## Uninstalling the Chart

```console
helm uninstall my-release --namespace my-namespace
```

## Configuration and installation details

### App settings

`settings` is a map of environment variables of the app container. Values go
through `tpl`, so they can refer to other values. Put secrets in `extraEnv`
(`valueFrom.secretKeyRef`) or `extraEnvFrom`, never in `settings`.

### Private images

If the image is private, create a pull secret and set it through
`global.imagePullSecrets`:

```console
kubectl create secret docker-registry regcred --docker-server=ghcr.io \
  --docker-username=<user> --docker-password=<token with read:packages>
```

```yaml
global:
  imagePullSecrets:
    - regcred
```

### Network policy

With `networkPolicy.enabled`, only the pods labelled
`<fullname>-client: "true"`, the ingress controller (when `ingress.enabled`)
and `networkPolicy.extraIngress` reach the app. Set
`networkPolicy.allowExternalEgress: false` to limit egress to DNS and
`networkPolicy.extraEgress`.

## Dependencies

| Repository | Name | Version |
|------------|------|---------|
| oci://registry-1.docker.io/bitnamicharts | common | 2.41.0 |

## Parameters

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| affinity | object | `{}` |  |
| enableServiceLinks | bool | `false` | inject the environment variables of every Service of the namespace into the pods (docker links style); off, use DNS names instead |
| extraEnv | array | `[]` | extra env entries for the pods, appended after `settings`: any name, plain `value` or `valueFrom`, e.g. a key of an existing Secret. Must not repeat a `settings` name |
| extraEnvFrom | array | `[]` | extra envFrom sources for the pods. An `env` entry of the same name wins |
| extraVolumeMounts | list | `[]` | extra volume mounts to be added to the container |
| extraVolumes | list | `[]` | extra volumes to be added to the pod |
| fullnameOverride | string | `""` |  |
| global.imagePullSecrets | list | `[]` | global images pull secrets |
| global.imageRegistry | string | `nil` | global images registry |
| hpa.averageUtilization | int | `70` | threshold in percentage of CPU utilization that triggers scaling |
| hpa.behavior | object | `{"scaleDown":{"policies":[{"periodSeconds":180,"type":"Pods","value":1}],"stabilizationWindowSeconds":180}}` | behavior of the HPA |
| hpa.enabled | bool | `false` | whether to enable the HorizontalPodAutoscaler |
| hpa.maxReplicaCount | int | `3` | maximum replica count |
| hpa.minReplicaCount | int | `1` | minimum replica count |
| image.pullPolicy | string | `"IfNotPresent"` | image pull policy |
| image.pullSecrets | array | `[]` | image pull secrets of this chart, added to global.imagePullSecrets |
| image.registry | string | `"ghcr.io/azavalon"` | image registry |
| image.repository | string | `"rust-app-template"` | image repository |
| image.tag | string | `"0.0.0"` | image tag, set at release time |
| ingress.enabled | bool | `false` | expose the app through the Gateway API (HTTPRoute) |
| ingress.gateway.annotations | map | `{}` | Gateway annotations |
| ingress.gateway.className | string | `"traefik"` | GatewayClass |
| ingress.gateway.enabled | bool | `false` | create a Gateway for this app; otherwise route.parentRefs |
| ingress.gateway.listeners | array | `[{"allowedRoutes":{"namespaces":{"from":"Same"}},"name":"web-{{ include \"common.names.fullname\" $ }}","port":8000,"protocol":"HTTP"}]` | Gateway listeners; the HTTPRoute attaches to the first one |
| ingress.hostname | string | `""` | hostname of the route; required when enabled |
| ingress.route | object | `{"annotations":{},"extraRules":[],"labels":{},"parentRefs":[],"pathPrefix":""}` | Gateway API HTTPRoute resource configuration |
| ingress.route.annotations | map | `{}` | Annotations to add to the HTTPRoute metadata |
| ingress.route.extraRules | array | `[]` | Extra Rules to add to the HTTPRoute |
| ingress.route.labels | map | `{}` | Labels to add to the HTTPRoute metadata |
| ingress.route.parentRefs | array | `[]` | Parent Gateway references; if empty and gateway.enabled is true, uses the created Gateway |
| ingress.route.pathPrefix | string | `""` | Optional path prefix the app is served under (e.g. "/app"); stripped before the request reaches the app |
| lifecycle | object | `{"preStop":{"enabled":true,"sleepSeconds":5}}` | Lifecycle hooks for the main container |
| lifecycle.preStop.enabled | bool | `true` | Enable preStop sleep hook |
| lifecycle.preStop.sleepSeconds | int | `5` | Number of seconds to sleep in preStop hook before SIGTERM handling proceeds Allows kube-proxy to gracefully remove the endpoints from iptables |
| livenessProbe.enabled | bool | `true` |  |
| livenessProbe.failureThreshold | int | `3` |  |
| livenessProbe.initialDelaySeconds | int | `5` |  |
| livenessProbe.path | string | `"/healthz"` | HTTP path of the probe |
| livenessProbe.periodSeconds | int | `20` |  |
| livenessProbe.successThreshold | int | `1` |  |
| livenessProbe.timeoutSeconds | int | `1` |  |
| nameOverride | string | `""` |  |
| networkPolicy | object | `{"allowExternal":false,"allowExternalEgress":true,"enabled":false,"extraEgress":[],"extraIngress":[],"ingressNSMatchLabels":{"kubernetes.io/metadata.name":"ingress-traefik"},"ingressNSPodMatchLabels":{"app.kubernetes.io/name":"traefik"}}` | NetworkPolicy of the pods. By default only the pods with the `<fullname>-client: "true"` label, the ingress controller (when ingress.enabled) and extraIngress get in; egress stays open. |
| networkPolicy.allowExternal | bool | `false` | accept connections from any source |
| networkPolicy.allowExternalEgress | bool | `true` | allow the pods to reach anything. When false, only DNS and extraEgress. |
| networkPolicy.enabled | bool | `false` | create the NetworkPolicy |
| networkPolicy.extraEgress | array | `[]` | extra egress rules |
| networkPolicy.extraIngress | array | `[]` | extra ingress rules |
| networkPolicy.ingressNSMatchLabels | map | `{"kubernetes.io/metadata.name":"ingress-traefik"}` | labels of the namespace of the ingress controller, let in when ingress.enabled |
| networkPolicy.ingressNSPodMatchLabels | map | `{"app.kubernetes.io/name":"traefik"}` | labels of the ingress controller pods in that namespace |
| nodeSelector | object | `{}` |  |
| pdb | object | `{"create":false,"maxUnavailable":"","minAvailable":""}` | PodDisruptionBudget, created only with more than one replica or with the HPA |
| pdb.create | bool | `false` | create the PodDisruptionBudget |
| pdb.maxUnavailable | string | `""` | maximum number or percentage of pods that may be unavailable; defaults to 1 when both are empty |
| pdb.minAvailable | string | `""` | minimum number or percentage of pods that must stay available |
| podAnnotations | map | `{}` | pod annotations |
| podSecurityContext | map | `{"fsGroup":1001,"runAsGroup":1001,"runAsNonRoot":true,"runAsUser":1001,"seccompProfile":{"type":"RuntimeDefault"}}` | pod security context |
| readinessProbe.enabled | bool | `true` |  |
| readinessProbe.failureThreshold | int | `5` |  |
| readinessProbe.initialDelaySeconds | int | `5` |  |
| readinessProbe.path | string | `"/healthz"` | HTTP path of the probe |
| readinessProbe.periodSeconds | int | `5` |  |
| readinessProbe.successThreshold | int | `1` |  |
| readinessProbe.timeoutSeconds | int | `1` |  |
| replicaCount | int | `1` |  |
| resources.limits.memory | string | `"128Mi"` |  |
| resources.requests.cpu | string | `"100m"` |  |
| resources.requests.memory | string | `"128Mi"` |  |
| securityContext | map | `{"allowPrivilegeEscalation":false,"capabilities":{"drop":["ALL"]},"readOnlyRootFilesystem":true}` | container security context |
| service.port | int | `80` | service port |
| service.targetPort | int | `8000` | container port the app listens on |
| service.type | string | `"ClusterIP"` | service type |
| serviceAccount.annotations | map | `{}` | ServiceAccount annotations (e.g. cloud IAM role bindings) |
| serviceAccount.automount | bool | `false` | mount the API token in the pods; only for apps that call the Kubernetes API |
| serviceAccount.create | bool | `false` | create a ServiceAccount |
| serviceAccount.name | string | `""` | name of the ServiceAccount; the full name when empty and create is true |
| settings | object | `{"APP_PORT":"{{ .Values.service.targetPort }}","RUST_BACKTRACE":"1"}` | Environment variables of the app container. Values go through `tpl`. |
| settings.APP_PORT | string | `"{{ .Values.service.targetPort }}"` | port the app listens on; keep it equal to service.targetPort |
| settings.RUST_BACKTRACE | string | `"1"` | print a backtrace when the app panics |
| terminationGracePeriodSeconds | int | `30` | Grace period before Kubernetes force-kills the container during termination |
| tmpSizeLimit | string | `"64Mi"` | size limit of the writable /tmp (the root filesystem is read-only) |
| tolerations | list | `[]` |  |
| updateStrategy | object | `{"type":"RollingUpdate"}` | update strategy |

----------------------------------------------
Autogenerated from chart metadata using [helm-docs v1.14.2](https://github.com/norwoodj/helm-docs/releases/v1.14.2)
