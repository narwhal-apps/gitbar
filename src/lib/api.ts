import type { AuthState, GetAccessTokenArgs, GetAccessTokenResponse, GithubSettings, Review, User } from '../types';
import { fetch } from '@tauri-apps/plugin-http';

export async function getAccessToken({ clientId, clientSecret, code, hostname }: GetAccessTokenArgs) {
  

  const res = await fetch<GetAccessTokenResponse>(`https://${hostname}/login/oauth/access_token`, {
    method: 'POST',
    headers: {
      Accept: 'application/json',
    },
    body: JSON.stringify({
      client_id: clientId,
      client_secret: clientSecret,
      code,
    }),
    responseType: 'json',
  });

  if (!res.ok) {
    throw res;
  }

  return res;
}

export const getUserData = async (token: string, hostname: string): Promise<User> => {
  const response = await fetch(`https://api.${hostname}/user`, {
    method: 'GET',
    headers: { Accept: 'application/json', Authorization: `token ${token}`, },
  });

  const { data } = response; 

  return {
    id: data.id,
    login: data.login,
    name: data.name,
    avatar_url: data.avatar_url,
    html_url: data.html_url,
    company: data.company,
    email: data.email,
  };
};

export const getReviews = async (account: AuthState, settings: GithubSettings): Promise<Review> => {
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
  const client = await getClient();
  const response: {
    data: {
      data: {
        search: Review;
      };
    };
  } = await client.post(`https://api.${account.hostname}/graphql`, Body.text(JSON.stringify(body)), {
    headers: {
      Authorization: `token ${account.token}`,
    },
  });
  const { data } = response;
  return data.data.search;
};

export const getOrganizations = async (account: AuthState): Promise<string[]> => {
  const client = await getClient();
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
  } = await client.post(`https://api.${account.hostname}/graphql`, Body.text(JSON.stringify(body)), {
    headers: {
      Authorization: `token ${account.token}`,
    },
  });

  const { data } = response;
  const orgs = data.data.viewer.organizations.nodes.map(org => org.login);
  orgs.unshift(data.data.viewer.login);
  return orgs;
};
