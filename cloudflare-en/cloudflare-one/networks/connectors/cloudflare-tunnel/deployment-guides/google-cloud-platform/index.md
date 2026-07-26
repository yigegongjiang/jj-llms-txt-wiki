---
description: GCP in Zero Trust networking.
title: GCP
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/cloudflare-one/llms.txt  
> Use this file to discover all available pages before exploring further.

# GCP

Last updated Apr 17, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/deployment-guides/google-cloud-platform/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

This guide covers how to connect a Google Cloud Project (GCP) virtual machine to Cloudflare using our lightweight connector, `cloudflared`.

We will deploy:

* A Google Cloud Project (GCP) virtual machine that runs a basic HTTP server.
* A Cloudflare Tunnel that allows users to connect to the service via either a public hostname or a private IP address.

Tip

If your server is behind a restrictive firewall, verify it can reach Cloudflare on port `7844` before proceeding. Refer to [Connectivity pre-checks](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/troubleshoot-tunnels/connectivity-prechecks/).

### Prerequisites

To complete the following procedure, you will need to:

* [Add a website to Cloudflare](https://developers.cloudflare.com/fundamentals/manage-domains/add-site/)
* [Deploy the Cloudflare One Client](https://developers.cloudflare.com/cloudflare-one/team-and-resources/devices/cloudflare-one-client/deployment/manual-deployment/) on an end-user device

## 1\. Create a VM instance in GCP

1. In your [Google Cloud Console ↗](https://console.cloud.google.com/), [create a new project ↗](https://developers.google.com/workspace/guides/create-project).
2. Go to **Compute Engine** \> **VM instances**.
3. Select **Create instance**.
4. Name your VM instance. In this example we will name it `http-test-server`.
5. Choose your desired operating system and specifications. For this example, you can use the following settings:

  * **Machine family:** General Purpose
  * **Series:** E2
  * **Machine type:** e2-micro
  * **Boot disk image:** Debian GNU/Linux 12
  * **Firewalls**: Allow HTTP and HTTPS traffic
6. Under **Advanced options** \> **Management** \> **Automation**, add the following startup script. This example deploys a basic Apache web server on port `80`.  
```bash  
#!/bin/bash  
apt update  
apt -y install apache2  
cat <<EOF > /var/www/html/index.html  
<html><body><h1>Hello Cloudflare!</h1>  
<p>This page was created for a Cloudflare demo.</p>  
</body></html>  
EOF  
```
7. Select **Create**.
8. The operating system automatically starts the Apache HTTP server. To verify that the server is running:

  1. Copy the **External IP** for the VM instance.
  2. Open a browser and go to `http://<EXTERNAL IP>`. You should see the **Hello Cloudflare!** test page.
9. To login to the VM instance, open the dropdown next to **SSH** and select _Open in browser window_.

## 2\. Create a Cloudflare Tunnel

Create a Cloudflare Tunnel and run it on the GCP VM.

1. Log in to the Cloudflare dashboard and go to **Networking** \> **Tunnels**.  
[Go to **Tunnels** ↗](https://dash.cloudflare.com/?to=/:account/tunnels)
2. Select **Create a tunnel**.
3. Enter a name for your tunnel (for example, `gcp-tunnel`).
4. Select **Create Tunnel**.
5. Choose your operating system (select **Debian** for your GCP VM). Copy the installation command and run it on your GCP VM.
6. Wait for the tunnel to connect. Once the connection is established, select **Continue**.

## 3\. Connect using a public hostname

[Published applications](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/routing-to-tunnel/) allow anyone on the Internet to connect to HTTP resources hosted on your virtual private cloud (VPC). To add a published application for your Cloudflare Tunnel:

1. On the **Routes** tab, select **Add route**, then select **Published application**.
2. Enter a hostname for the application (for example, `hellocloudflare.<your-domain>.com`).
3. Under **Service**, enter `http://localhost:80`.
4. Select **Add route**.
5. To test, open a browser and go to `http://hellocloudflare.<your-domain>.com`. You should see the **Hello Cloudflare!** test page.

You can optionally [create an Access application](https://developers.cloudflare.com/cloudflare-one/access-controls/applications/http-apps/self-hosted-public-app/) to control who can access the service.

## 4\. Connect using a private IP

[Private network routes](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/private-net/cloudflared/) allow users to connect to your VPC network using the Cloudflare One Client. To add a private network route for your Cloudflare Tunnel:

1. In the Cloudflare dashboard, go to **Networking** \> **Routes**.  
[Go to **Routes** ↗](https://dash.cloudflare.com/?to=/:account/magic-networks/routes)
2. Select **Create route** \> **Tunnel CIDR**. Select the tunnel you just created, enter the **Private IP address** of your GCP VM (for example, `10.0.0.4`), and select **Create route**. You can expand the IP range later if necessary.
3. In your [Split Tunnel configuration](https://developers.cloudflare.com/cloudflare-one/team-and-resources/devices/cloudflare-one-client/configure/route-traffic/split-tunnels/#add-a-route), make sure the private IP is routing through the Cloudflare One Client. For example, if you are using Split Tunnels in **Exclude** mode, delete `10.0.0.0/8`. We recommend re-adding the IPs that are not explicitly used by your GCP VM.  
To determine which IP addresses to re-add, subtract your GCP VM IPs from `10.0.0.0/8`:  
Base CIDRSubtracted CIDRs  
Calculate  
Add the results back to your Split Tunnel Exclude mode list.
4. To test on a user device:

  1. [Log in to the Cloudflare One Client](https://developers.cloudflare.com/cloudflare-one/team-and-resources/devices/cloudflare-one-client/deployment/manual-deployment/).
  2. Open a terminal window and connect to the service using its private IP:  
```sh  
curl 10.0.0.4  
```  
```txt  
<html><body><h1>Hello Cloudflare!</h1>  
<p>This page was created for a Cloudflare demo.</p>  
</body></html>  
```

You can optionally [create Gateway network policies](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/private-net/cloudflared/#4-recommended-filter-network-traffic-with-gateway) to control who can access the GCP VM via its private IP.

## Firewall configuration

To secure your VM instance, you can [configure your VPC firewall rules ↗](https://cloud.google.com/firewall/docs/using-firewalls) to deny all ingress traffic and allow only egress traffic to the [Cloudflare Tunnel IP addresses](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/configure-tunnels/tunnel-with-firewall/#required-for-tunnel-operation). Since GCP denies ingress traffic by [default ↗](https://cloud.google.com/firewall/docs/firewalls#default%5Ffirewall%5Frules), you can delete all ingress rules and leave only the relevant egress rules.

Note

If you delete the default `allow-ssh` rule, you will be unable to SSH back into the VM.

After configuring your VPC firewall rules, verify that you can still access the service through Cloudflare Tunnel via its [public hostname](#3-connect-using-a-public-hostname) or [private IP](#4-connect-using-a-private-ip). The service should no longer be accessible from outside Cloudflare Tunnel -- for example, if you go to `http://<EXTERNAL IP>` the test page should no longer load.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/deployment-guides/google-cloud-platform/#page","headline":"GCP · Cloudflare One docs","description":"GCP in Zero Trust networking.","url":"https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/deployment-guides/google-cloud-platform/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-17","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"},"keywords":["GCP"]}
```
