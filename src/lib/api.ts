import type { AuthState, GetAccessTokenArgs, GetAccessTokenResponse, GithubSettings, Review, User } from '../types';
import { fetch } from '@tauri-apps/plugin-http';
import { createURL } from './url';

const GITHUB_AUTHORIZE_ENDPOINT = 'https://github.com/login/oauth/authorize';
const GITHUB_AUTH_SCOPES = ['repo', 'read:user'];

export function createAuthURL(port: number) {
  const GITHUB_AUTH_QUERIES = {
    client_id: import.meta.env.VITE_CLIENT_ID,
    scope: GITHUB_AUTH_SCOPES.join(' '),
    redirect_uri: `http://localhost:${port}/callback`,
  };

  return createURL({ url: GITHUB_AUTHORIZE_ENDPOINT, query: GITHUB_AUTH_QUERIES });
}


export async function getAccessToken({ clientId, clientSecret, code, hostname }: GetAccessTokenArgs) {
  const res = await fetch(`https://${hostname}/login/oauth/access_token`, {
    method: 'POST',
    headers: {
      Accept: 'application/json',
    },
    body: JSON.stringify({
      client_id: clientId,
      client_secret: clientSecret,
      code,
    }),
  });

  if (!res.ok) {
    throw res;
  }

  return await res.json();
}

export const getUserData = async (token: string, hostname: string): Promise<User | void> => {
  try {
    const res = await fetch(`https://api.${hostname}/user`, {
      method: 'GET',
      headers: { Accept: 'application/json', Authorization: `token ${token}` },
    });

    if (!res.ok) {
      throw res;
    }

    const data = await res.json();

    return {
      id: data.id,
      login: data.login,
      name: data.name,
      avatar_url: data.avatar_url,
      html_url: data.html_url,
      company: data.company,
      email: data.email,
    };
  } catch (err) {
    console.error('err :', err);
  }
};

export const getReviews = async (account: AuthState, settings: GithubSettings): Promise<Review> => {
  console.log('settings :', settings);
  console.log('account :', account);
  const orgs = settings.organizations?.map(org => `user:${org}`).join(' ');
  let search = `type:pr state:${settings.state} archived:${settings.archive} ${settings.type}:${account.user?.login}`;
  if (orgs) {
    search += ` ${orgs}`;
  }
  const text = `
  {
    search(query: "${search}", type: ISSUE, first: 100) {
      issueCount
      edges {
        node {
          ... on PullRequest {
            repository {
              nameWithOwner
            }
            author {
              login
              __typename
            }
            createdAt
            number
            url
            title
            merged
            closed
            isDraft
            reviewDecision
            totalCommentsCount
            labels(first: 10) {
              edges {
                node {
                  color
                  name
                }
              }
            }
            statusCheckRollup {
              state
            }
            isReadByViewer
          }
        }
      }
    }
  }
`;
  const body = {
    query: text,
  };
  // const client = await getClient();
  const response = await fetch(`https://api.${account.hostname}/graphql`, {
    method: 'POST',
    headers: {
      Authorization: `token ${account.token}`,
    },
    body: JSON.stringify(body),
  });
  const data = await response.json();
  console.log('data :', data);
  return data.data.search;
};

export const getOrganizations = async (account: AuthState): Promise<string[]> => {
  const text = `
  {
    viewer {
      login
      organizations(first: 10) {
        nodes {
          login
        }
      }
    }
  }
`;
  const body = {
    query: text,
  };
  const response: {
    data: {
      data: {
        viewer: {
          login: string;
          organizations: {
            nodes: Array<{
              login: string;
            }>;
          };
        };
      };
    };
  } = await fetch(`https://api.${account.hostname}/graphql`, {
    method: 'POST',
    headers: {
      Authorization: `token ${account.token}`,
    },
    body: JSON.stringify(body),
  });

  const data = await response.json();
  const orgs = data.data.viewer.organizations.nodes.map(org => org.login);
  orgs.unshift(data.data.viewer.login);
  return orgs;
};
