![build](https://github.com/jossware/kubectl-node-provider-id/actions/workflows/build.yaml/badge.svg)

# kubectl-node-provider-id

Sometimes you just need to know the instance ID of a node in your managed
Kubernetes cluster. Maybe you need it to correlate some observability data or to
use in another command or just for troubleshooting something. This plugin just
makes it slightly easier to get that information when you need it.

Most cloud providers [populate][node-spec] the `spec.providerID` field on
Kubernetes `Node` resources in the format:
`<ProviderName>://<ProviderSpecificNodeID>`. It is not visible in the default
`kubectl get nodes` output, even with `-o wide`. You can get it with `kubectl
get nodes -o jsonpath='{.items[*].spec.providerID}'` (or `-o json | jq`
equivalent), but that's a bit unwieldy. Even if you do that, you may want to
process it even further. For example, a `providerID` for a node in AWS might
look like `aws://us-west-2a/i-0a1b2c3d4e5f6g7h8`. You might just care about the
instance ID.

This plugin can display the provider ID in your desired format.

By default, it will display the entire provider ID.

``` shell
$ kubectl node-provider-id
NODE                      PROVIDER ID
node-prov-control-plane   kind://podman/node-prov/node-prov-control-plane
node-prov-worker          kind://podman/node-prov/node-prov-worker
node-prov-worker2         kind://podman/node-prov/node-prov-worker2
```

> [!NOTE]
> Use the `--context` (or `-c`) flag to target a different cluster.

You can use the `--template` (or `-t`) flag to format the output.

``` shell
$ kubectl node-provider-id -t "{:last}"
NODE                      PROVIDER ID
node-prov-control-plane   node-prov-control-plane
node-prov-worker          node-prov-worker
node-prov-worker2         node-prov-worker2
```

In the above example, the `{:last}` template displays the last segment of the
provider ID. See [Templates](#template) for more information on how to use them.

You can use the `--format` (or `-f`) flag to format the output. It supports
table (default), plain, JSON, and YAML output formats.

``` shell
# plain
kubectl node-provider-id -oplain
kind://podman/node-prov/node-prov-control-plane
kind://podman/node-prov/node-prov-worker
kind://podman/node-prov/node-prov-worker

# json
$ kubectl node-provider-id -t "{:last}" -ojson
[{"name":"node-prov-control-plane","provider_id":"node-prov-control-plane"},{"name":"node-prov-worker","provider_id":"node-prov-worker"},{"name":"node-prov-w orker2","provider_id":"node-prov-worker2"}]

# yaml
$ kubectl node-provider-id -oyaml
- name: node-prov-control-plane
  provider_id: kind://podman/node-prov/node-prov-control-plane
- name: node-prov-worker
  provider_id: kind://podman/node-prov/node-prov-worker
- name: node-prov-worker2
  provider_id: kind://podman/node-prov/node-prov-worker2
```

## Templates

You can use a template to define how you want information extracted from
`.spec.providerID`. The plugin will parse the `providerID` value and make the
discovered information available via tokens you can use in your templates. It
splits the value of the ID (the part after the provider "protocol") by "/" and
it is possible to access individual parts by index or by named helpers.

Example with an AWS Provider ID: "aws://us-west-2/i-0abcdef1234567890".

| Token       | Value                               |
|-------------|-------------------------------------|
| {:provider} | aws                                 |
| {:last}     | i-0abcdef1234567890                 |
| {:first}    | us-west-2                           |
| {:all}      | us-west-2/i-0abcdef1234567890       |
| {:url}      | aws://us-west-2/i-0abcdef1234567890 |
| {0}         | us-west-2                           |
| {1}         | i-0abcdef1234567890                 |
| {:node}     | ip-192-168-1-123.ec2.internal       |

You can use other characters in your template, of course. For example, you could
output a CSV of nodes and their instance IDs:

``` shell
$ kubectl node-provider-id -t "{:node},{:last}" -oplain
ip-192-168-1-123.ec2.internal,i-0abcdef1234567890
ip-192-168-1-124.ec2.internal,i-00987654321fedcba
ip-192-168-1-125.ec2.internal,i-0a34567dcb2890ef1
```

## Configuration

You can define the default template and output format in a yaml configuration
file. For example:

``` shell
cat << EOF > ~/.config/kubectl-node-provider-id.yaml
template: "{:last}"
format: plain
EOF
```

Now, the plugin will use the plain output format with the above template by
default.

``` shell
% kubectl node-provider-id
node-prov-control-plane
node-prov-worker
node-prov-worker2
```

## More fun with `.spec.providerID`

If you want to propagate data from the provider ID into `Node` labels or
annotations, check out [node-provider-labeler][node-provider-labeler].

[node-spec]: https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#NodeSpec
[node-provider-labeler]: https://github.com/jossware/node-provider-labeler/
