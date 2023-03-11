import { Octokit } from "octokit";
const { who, num, auth } = {
  who: process.argv[2],
  num: process.argv[3],
  auth: process.argv[4]
}
const owner = "tTdly-Old";
const repo = "Blog"
const octokit = new Octokit({ auth : "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"});
async function close_and_Lock(){
  await octokit.request('PATCH /repos/{owner}/{repo}/issues/{issue_number}', {
    owner,
    repo,
    issue_number:num,
    state: "closed",
    state_reason: "bad-issue",
    headers: {
      'X-GitHub-Api-Version': '2022-11-28'
    }
  });
  await octokit.request('PUT /repos/{owner}/{repo}/issues/{issue_number}/lock', {
    owner,
    repo,
    issue_number: num,
    lock_reason: 'bad-issue',
    headers: {
      'X-GitHub-Api-Version': '2022-11-28'
    }
  })

}
main();