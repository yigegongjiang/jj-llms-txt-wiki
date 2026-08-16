---
description: Review and maintain documentation quality.
title: Content reviews
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Content reviews

Last updated Aug 4, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/how-we-docs/reviews/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

We work (and appreciate working) in [GitHub ↗](https://github.com/cloudflare/cloudflare-docs), but it's not a perfect tool by any means.

We've added several ergonomic improvements to help us triage incoming work, streamline reviews, and automate communication.

## Triage work

To help our writers triage work (and help with backend reporting), we automate a few labels and issue / pull request assignment.

### Pull requests

For pull requests, we use a specific [GitHub action ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/pr-label-assign.yml) to add labels and assign codeowners.

* [Label products ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/actions/label-products/): We add labels for the top-level product folder, which helps writers scan incoming pull requests and see which are relevant to them.
* [Label size ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/actions/label-size/): We add labels for the size of the pull request, which helps writers see the relative size of the difference. It's not a perfect measure, but we use the lines changed to estimate the size of the pull request:
* [Assign codeowners ↗](https://github.com/cloudflare/cloudflare-docs/tree/production/.github/actions/assign-pr): We use our [CODEOWNERS ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/CODEOWNERS) file to automatically assign people to pull requests based on the files changed. This assignment helps writers scan and filter to see which pull requests are relevant to them.

### Issues

For issues, we use a similar [GitHub action ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/issue-label-assign.yml) to add labels and assign codeowners.

Our [script ↗](https://github.com/cloudflare/cloudflare-docs/tree/production/.github/actions/issue-label-assign) treats issues because usually they contain links to our site instead of files within the repository. We grab the associated links in the issue description fields and then use those to assign writers (again based on `CODEOWNERS`) and add product labels.

## Streamline reviews

To streamline reviews, we use several automations to help solve common reviewer issues.

### Will this break anything?

We have one required check that runs on every commit, [CI ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/ci.yml).

This check makes sure that:

* The site builds correctly.
* All [internal links](https://developers.cloudflare.com/style-guide/how-we-docs/links/#internal-links) are valid.
* There are no [infinite redirects](https://developers.cloudflare.com/style-guide/how-we-docs/redirects/#infinite-redirects).
* Specific pages and functionality in our docs [behave as expected ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/worker/index.worker.test.ts).

### Who needs to approve this?

We commonly get questions about approvals, especially for pull requests that touch multiple product areas (or our components).

We have a specific part of [CI ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/ci.yml#L32) that [posts a comment ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/bin/post-codeowners-comment/index.ts) with the relevant codeowners.

![Codeowners comment](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=2288,height=818,format=webp/_astro/codeowners-comment.DDCh7twA.png) 

### What changed?

We use a [GitHub action ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/publish-preview.yml) to publish preview builds for every commit on a pull request (and [comment those links ↗](https://github.com/cloudflare/cloudflare-docs/tree/production/bin/post-preview-url-comment) on the pull request). This action ensures that reviewers can see exactly what the site and content will look like when they're rendered (and not just look at the changed markdown in GitHub).

We are in the process of migrating to [Workers Builds](https://developers.cloudflare.com/workers/ci-cd/builds/git-integration/github-integration/), which provides that functionality natively.

Based on feedback, we have also added a before/after table of links to help reviewers easily find links in the preview builds.

![Before/after table](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=2280,height=666,format=webp/_astro/preview-comment.Bgnu4w0p.png) 

### Is there anything else I need to check?

One difficult thing to check is potential redirects when a pull request renames or deletes content file paths.

We have a [specific action ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/comment-changed-filenames.yml) that posts a comment to help reviewers identify and then check these paths.

![GitHub Actions redirect comment](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=1239,height=442,format=webp/_astro/redirects-github.D5I7CV0r.png) 

## Automate communication

We automate communication primarily through the [no-response ↗](https://github.com/lee-dohm/no-response) GitHub Action.

Being open source means that we accept issues and pull requests from anyone! And a lot of these are either self explanatory or have enough context for us to follow up on them.

The ones without enough context, however, are often painful (especially in a [busy repo ↗](https://github.com/cloudflare/cloudflare-docs/pulse) like ours). A writer has to ask for more detail, then remember to check back in, and then sometimes re-ask for more detail, and then check in again.

To help avoid some of this mental toil, we'll ask a question and then apply the `more-information-needed` label. This label starts a [14-day clock ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/no-response.yml) for the author to respond. If they do respond, the label gets removed and the conversation can continue. If they don't respond, the issue automatically gets closed with a [comment explaining why ↗](https://github.com/cloudflare/cloudflare-docs/issues/22943#issuecomment-3002211164).

![No response comment](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=2398,height=572,format=webp/_astro/no-response-comment.BLo5VxlU.png) 

This workflow - we hope - balances the needs our of team with a healthy respect for our contributors.

### Stale issues

We intentionally avoid the [stale workflow ↗](https://github.com/github/docs/blob/main/.github/workflows/stale.yml) that closes pull requests or issues that have been inactive for a specific period of time.

In our opinion, this workflow causes more friction and frustration than it solves. Just because something has been around for a year doesn't mean it's not still relevant.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/how-we-docs/reviews/#page","headline":"Content reviews · Cloudflare Style Guide","description":"Review and maintain documentation quality.","url":"https://developers.cloudflare.com/style-guide/how-we-docs/reviews/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-04","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
