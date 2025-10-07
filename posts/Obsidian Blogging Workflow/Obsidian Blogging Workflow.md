---
tags:
  - blog-post
  - draft
date: 2025-10-07
title: "Blogging: Obsidian to Hugo Workflow"
draft: false
---

## Summary

This is my first blogpost and im going to talk about how to make an easy workflow between [[Obsidian]] and [[Hugo]]. I love using [[Obsidian]] and I love taking Notes, now its time to share them with the rest of the Internet. The approach I take is a three stage approach, which is outlined in the diagram below. We will walk our way through this workflow in this blog and outline the tools and the stages as we go through.

![[Obsidian Blog Workflow.excalidraw]]
## What tools you need
The tools and languages you will need during this blog are shown below:

- [[git]] - versioning your blogs and Obsidian Notes
- [[GitHub]] - Hosting your Repositories and publishing your Hugo Blog
- [[Markdown]] - The file format of your blogs
- [[Hugo]] - Turning your [[Markdown]] files into beautiful Web code ([[JavaScript]],[[css]],[[html]])
	- [[GoLang]] - The language used to create the [[Static Site Generator]] 


# The Project Structure

Before we go into the different stages of the workflow. Its best to first discuss the repository structure we are using. I have the following repositories

- Gibb Knowledge Base - A private repository that contains my personal notes and is self hosted.
- [Blog Posts](https://github.com/ScottGibb/blog-posts)- A public repository that contains my blog posts in markdown format. This doesn't contain any specific [[Hugo]] files, it will primarily consist of [[Markdown]] and images. 
- [[Gibbiverse]](https://github.com/ScottGibb/Gibbiverse) - A public repository that contains my [[Hugo]] site. This is where the configuration of the site and theme settings are stored. 

The `Blog Posts` repository is cross linked between the two repositories. Allowing it to be used in both places. The idea is to write your blogs in [[Obsidian]] and then have them referenced in the [[Hugo]] Site Repository

# The Stages
## [[Obsidian]]
In this stage you make notes as usual and write your blog posts. The way in which I do this is by having a dedicated folder in my Obsidian Vault called `Blog Posts`. This is where all my blog posts are written. I regularly push these notes to the `Blog Posts` repository. I also have a folder called `drafts` where I write my drafts before they are ready to be moved to the `Blog Posts` folder.

## Hugo 

## Updating Submodules

Once im comfortable with the blog post, I then move to the [[Hugo]] repository. In this repository I update the `Blog Posts` submodule by first creating a new branch and then pulling in the latest changes from the `Blog Posts` repository.  During this time I can see what the Blog would like by running the following command in the root of the repository

```bash
hugo server -D
```

Once me and the [[CICD]]  system ([[MegaLinter]] is used to check for linting across the project space) are happy with the results, I go on to merge this back into main.

## Committing to a Release

The workflow Im using for releasing the blog is the one provided by [[Hugo]] and uses [[../../../../Resources/Version Control/GitHub/GitHub Actions|GitHub Actions]] alongside [[GitHub Pages]] to create both a [[../../../../Resources/Version Control/GitHub/GitHub|GitHub]] release alongside a published website. This workflow only runs on a tagged commit. As such I am in full control of when the next Blog post is published.