export type GitHubPR = {
  node: {
    repository: {
      nameWithOwner: string;
    };
    author: {
      login: string;
      __typename: 'User' | 'Bot';
    };
    createdAt: Date;
    number: string;
    url: string;
    title: string;
    merged: boolean;
    closed: boolean;
    isDraft: boolean;
    reviewDecision: 'APPROVED' | 'CHANGES_REQUESTED' | 'REVIEW_REQUIRED';
    totalCommentsCount: number;
    labels: {
      edges: Array<{
        node: {
          name: string;
          color: string;
        };
      }>;
    };
    statusCheckRollup: {
      state: 'SUCCESS' | 'FAILURE' | 'PENDING' | 'ERROR' | 'EXPECTED';
    };
    isReadByViewer: boolean;
  };
};

export type PullRequestState = 'open' | 'draft' | 'merged' | 'closed';
