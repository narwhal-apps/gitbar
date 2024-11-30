export type GitHubPR = {
  node: {
    repository: {
      nameWithOwner: string;
    };
    author: {
      login: string;
      __typename: "User" | "Bot";
    };
    createdAt: Date;
    number: string;
    url: string;
    title: string;
    merged: boolean;
    closed: boolean;
    isDraft: boolean;
    reviewDecision: "APPROVED" | "CHANGES_REQUESTED" | "REVIEW_REQUIRED";
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
      state: "SUCCESS" | "FAILURE" | "PENDING" | "ERROR" | "EXPECTED";
    };
    isReadByViewer: boolean;
  };
};

export type PullRequestState = 'open' | 'draft' | 'merged' | 'closed';

export function getPRState(pr: GitHubPR): PullRequestState {
  const { merged, closed, isDraft } = pr.node;
  if (merged) return 'merged';
  if (closed) return 'closed';
  if (isDraft) return 'draft';
  return 'open';
}

export function getStatusType(pr: GitHubPR): 'success' | 'pending' | 'failure' {
  const state = pr.node.statusCheckRollup?.state;
  switch (state) {
    case 'SUCCESS':
      return 'success';
    case 'FAILURE':
    case 'ERROR':
      return 'failure';
    default:
      return 'pending';
  }
}

export function formatDate(date: Date): string {
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));
  
  if (days === 0) return 'Today';
  if (days === 1) return 'Yesterday';
  if (days < 7) return `${days} days ago`;
  return new Date(date).toLocaleDateString();
}