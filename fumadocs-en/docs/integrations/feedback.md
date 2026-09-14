# Fumadocs (Framework Mode): Feedback

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/integrations/feedback.mdx

Receive feedback from your users

## Overview [#overview]

Feedback is crucial for knowing what your reader thinks, and help you to further improve documentation content.

You can integrate a simple feedback system with Fumadocs.

## Installation [#installation]

Install it using **Fumadocs CLI**, it also adds the component to your docs page when it finds one:

```npm
npx @fumadocs/cli feature feedback
```

Or install the component only:

```npm
npx @fumadocs/cli@latest add feedback
```

### Page Feedback [#page-feedback]

To create a page-level feedback UI, add the `<Feedback />` component to your docs page:

```tsx
import { DocsPage } from 'fumadocs-ui/layout/docs/page';
import { Feedback } from '@/components/feedback/client';

export default async function Page() {
  return (
    <DocsPage>
      {/* at the bottom of page */}
      <Feedback
        onSendAction={async (feedback) => {
          'use server';

          console.log(feedback);
        }}
      />
    </DocsPage>
  );
}
```

- `onSendAction`: fired when user submit feedback.

You can specify a server action, or any function (in client component) to handle the user feedback.
For example, to report user feedback as a `on_rate_docs` event on PostHog.

### Feedback Block [#feedback-block]

You can also configure block-level feedback (e.g. a feedback popover as user select text).

Add the `remark-block-id` Remark plugin:

```tsx title="source.config.ts (Fumadocs MDX)"
import {
  remarkBlockId,
  type RemarkBlockIdOptions,
} from 'fumadocs-core/mdx-plugins/remark-block-id';
import { defineConfig } from 'fumadocs-mdx/config';

const blockIdOptions: RemarkBlockIdOptions = {
  addDataAttribute: 'feedback',
};

export default defineConfig({
  mdxOptions: {
    remarkPlugins: [
      // [!code ++]
      [remarkBlockId, blockIdOptions],
    ],
  },
});
```

Then, wrap your page content under the `FeedbackText` component:

```tsx
import { DocsPage } from 'fumadocs-ui/layout/docs/page';
import { FeedbackText } from '@/components/feedback/client';

export default async function Page() {
  return (
    <DocsPage>
      <FeedbackText
        onSendAction={async (feedback) => {
          'use server';

          console.log(feedback);
        }}
      >
        {/* the content of page */}
      </FeedbackText>
    </DocsPage>
  );
}
```

- `onSendAction`: fired when user submit feedback.

<Callout type='info' title='Good to know'>

`remark-block-id` generates a block ID from its content and order in the page, hence it is also possible to track the blocks from 3rd party services.

</Callout>

### Integrating with GitHub Discussion [#integrating-with-github-discussion]

To report your feedback to GitHub Discussion, you can copy this file as a starting point:

```ts title='lib/github.ts'
import { App, Octokit } from 'octokit';
import {
  blockFeedback,
  BlockFeedback,
  pageFeedback,
  type ActionResponse,
  type PageFeedback,
} from '@/components/feedback/schema';

export const repo = 'fumadocs';
export const owner = 'fuma-nama';
export const DocsCategory = 'Docs Feedback';

let instance: Octokit | undefined;

async function getOctokit(): Promise<Octokit> {
  if (instance) return instance;
  const appId = process.env.GITHUB_APP_ID;
  const privateKey = process.env.GITHUB_APP_PRIVATE_KEY;

  if (!appId || !privateKey) {
    throw new Error('No GitHub keys provided for Github app, docs feedback feature will not work.');
  }

  const app = new App({
    appId,
    privateKey,
  });

  const { data } = await app.octokit.request('GET /repos/{owner}/{repo}/installation', {
    owner,
    repo,
    headers: {
      'X-GitHub-Api-Version': '2022-11-28',
    },
  });

  instance = await app.getInstallationOctokit(data.id);
  return instance;
}

interface RepositoryInfo {
  id: string;
  discussionCategories: {
    nodes: {
      id: string;
      name: string;
    }[];
  };
}

let cachedDestination: RepositoryInfo | undefined;
async function getFeedbackDestination() {
  if (cachedDestination) return cachedDestination;
  const octokit = await getOctokit();

  const {
    repository,
  }: {
    repository: RepositoryInfo;
  } = await octokit.graphql(`
  query {
    repository(owner: "${owner}", name: "${repo}") {
      id
      discussionCategories(first: 25) {
        nodes { id name }
      }
    }
  }
`);

  return (cachedDestination = repository);
}

export async function onPageFeedbackAction(feedback: PageFeedback): Promise<ActionResponse> {
  'use server';
  feedback = pageFeedback.parse(feedback);
  const url = new URL(feedback.url);

  return createDiscussionThread(
    url.pathname,
    `[${feedback.opinion}] ${feedback.message}\n\n> Forwarded from user feedback.`,
  );
}

export async function onBlockFeedbackAction(feedback: BlockFeedback): Promise<ActionResponse> {
  'use server';
  feedback = blockFeedback.parse(feedback);
  const url = new URL(feedback.url);
  url.hash = feedback.blockId;

  return createDiscussionThread(
    url.pathname,
    `> ${feedback.blockBody}\n\n${feedback.message}\n\n> [Forwarded from user feedback](${url.href}).`,
  );
}

async function createDiscussionThread(pageId: string, body: string) {
  const octokit = await getOctokit();
  const destination = await getFeedbackDestination();
  const category = destination.discussionCategories.nodes.find(
    (category) => category.name === DocsCategory,
  );

  if (!category) throw new Error(`Please create a "${DocsCategory}" category in GitHub Discussion`);

  const title = `Feedback for ${pageId}`;
  const queryResult: {
    search: {
      nodes: { id: string; title: string; url: string }[];
    };
  } = await octokit.graphql(`
          query {
            search(type: DISCUSSION, query: ${JSON.stringify(`"${title}" in:title repo:${owner}/${repo} author:@me`)}, first: 10) {
              nodes {
                ... on Discussion { id, title, url }
              }
            }
          }`);

  const discussion = queryResult.search.nodes.find((item) => item.title === title);

  if (discussion) {
    const result: {
      addDiscussionComment: {
        comment: { id: string; url: string };
      };
    } = await octokit.graphql(`
            mutation {
              addDiscussionComment(input: { body: ${JSON.stringify(body)}, discussionId: "${discussion.id}" }) {
                comment { id, url }
              }
            }`);

    return {
      githubUrl: result.addDiscussionComment.comment.url,
    };
  } else {
    const result: {
      createDiscussion: {
        discussion: { id: string; url: string };
      };
    } = await octokit.graphql(`
            mutation {
              createDiscussion(input: { repositoryId: "${destination.id}", categoryId: "${category.id}", body: ${JSON.stringify(body)}, title: ${JSON.stringify(title)} }) {
                discussion { id, url }
              }
            }`);

    return {
      githubUrl: result.createDiscussion.discussion.url,
    };
  }
}

```

1. Create your own GitHub App and obtain its app ID and private key.
2. Fill required environment variables.
3. Replace constants like `owner`, `repo`, and `DocsCategory`.
4. Use the `onPageFeedbackAction` & `onBlockFeedbackAction` in your feedback components.
