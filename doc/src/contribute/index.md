---
title: Contribute to Haneul
---

This page describes how to contribute to Haneul, and provides additional information about participating in the Haneul community.

You can find answers to common questions in our [FAQ](../contribute/faq.md).

## See our roadmap

Haneul is evolving quickly. See our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/DEVX_ROADMAP.md) for the planned updates over the next 30 days.

## Join the community

To connect with the Haneul community, join our [Discord](https://discord.gg/haneul).

## Open issues

To report an issue with Haneul, [create an issue](https://github.com/GeunhwaJeong/haneul/issues/new/choose) in the GitHub repo. Click **Get started** to open a template for the type of issue to create.


## Updates to docs

To request an update to a specific topic, click the **Source Code** link near the bottom of the page to open the source file in the GitHub repo. To submit a request, first choose the **Latest build** version of the doc site. This opens the main branch of the repo, which may contain a newer version of the topic than the one on **Devnet**.

Click **Edit this file**, make your changes, and then click **Propose changes** to create a pull request that includes your changes in a new branch.

## Install Haneul to contribute

To contribute to Haneul source code or documentation, you need only a GitHub account. You can commit updates and then submit a PR directly from the Github website, or create a fork of the repo to your local environment and use your favorite tools to make changes. Always submit PRs to the `main` branch.

### Create a fork

First, create a fork of the Haneul Labs Haneul repo in your own account so that you can work with your own copy.

**To create a fork using the website**

1. Log in to your Github account.
1. Browse to the [Haneul repo](https://github.com/GeunhwaJeong/haneul) on GitHub.
1. Choose **Fork** in the top-right, then choose **Create new fork**.
1. For **Owner**, select your username.
1. For **Repository name**, we suggest keeping the name haneul, but you can use any name. 
1. Optional. To contribute you need only the main branch of the repo. To include all branches, unselect the checkbox for **Copy the `main` branch only**.
1. Click **Create fork**.

### Clone your fork

Next, clone your fork of the repo to your local workspace.

**To clone your fork to your local workspace**
1. Open the GitHub page for your fork of the repo, then click **Sync fork**.
1. Click **Code**, then click **HTTPS** and copy the web URL displayed.
1. Open a terminal session and navigate to the folder to use, then run the following command, replacing the URL with the URL you copied from the Git page:

`git clone https://github.com/github-user-name/haneul.git`

The repo is automatically cloned into the `haneul` folder in your workspace.
Create a branch of your fork with following command (or follow the [GitHub topic on branching](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-and-deleting-branches-within-your-repository))

`Git checkout -b your-branch-name`

Use the following command to set the [remote upstream repo](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/configuring-a-remote-for-a-fork):

`git remote add upstream https://github.com/GeunhwaJeong/haneul.git`

You now have a fork of the Haneul repo set up in your local workspace. You can make changes to the files in the workspace, add commits, then push your changes to your fork of the repo to then create a Pull Request.

## Further reading

* Learn [about Haneul Labs](https://haneul-labs.com/) the company on our public site.
* Read the [Haneul Smart Contract Platform](../../paper/haneul.pdf) white paper.
* Implementing [logging](../contribute/observability.md) in Haneul to observe the behavior of your development.
* Find related [research papers](../contribute/research-papers.md).
* See and adhere to our [code of conduct](../contribute/code-of-conduct.md).
