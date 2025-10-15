---
tags:
  - blog-post
  - blogging
date: 2025-10-07
title: "Blogging: Obsidian to Hugo Workflow"
draft: true
---

## Summary

This is my first blogpost and im going to talk about how to make an easy workflow between Obsidian and [[Hugo]]. I love using [[Obsidian]] and I love taking Notes, now its time to share them with the rest of the Internet. The approach I take is a three stage approach, which is outlined in the diagram below. We will walk our way through this workflow in this blog and outline the tools and the stages as we go through.


## What tools will you need?

The tools and languages you will need during this blog are shown below:

- [[git]] - versioning your blogs and Obsidian Notes
- [GitHub](https://github.com/ScottGibb) - Hosting your Repositories and publishing your Hugo Blog
- [[Markdown]] - The file format of your blogs
- [[Hugo]] - Turning your Markdown files into beautiful Web code (JavaScript, css, html)
  - [[GoLang]] - The language used to create the Static Site Generator

# The Project Structure

Before we go into the different stages of the workflow. Its best to first discuss the repository structure we are using. I have the following repositories

- Gibb Knowledge Base - A private repository that contains my personal notes and is self hosted. This is where all my notes are stored and managed using Obsidian. I often link my posts to notes in here. However these links will not work on the published site.
- [Gibbiverse Content](https://github.com/ScottGibb/blog-posts)- A public repository that contains my blog posts in markdown format. This doesn't contain any specific Hugo files, it primarily consists of Markdown and images. No code is added here or project configuration. There is some [[../../../Resources/Version Control/GitHub/GitHub Actions|GitHub Actions]] added to allow [[MegaLinter]] to run and check the formatting.
- [Gibbiverse](https://github.com/ScottGibb/Gibbiverse) - A public repository that contains the Hugo site. This is where the configuration of the site and theme settings are stored. There are also another set of [[../../../Resources/Version Control/GitHub/GitHub Actions|GitHub Actions]] this time targeting the build and deploy process with [[GitHub Pages]].
- [Gibbiverse Link Fixer](https://github.com/ScottGibb/Gibbiverse) - A public repository that contains a [[../../../Resources/Languages/Python|Python]] application which is designed to replace the links inside the content so that the content renders correctly in [[Hugo]].

This whole structure allows me to continue using [[Obsidian]] to write content and link to my personal notes and vice versa, enhancing the Second Brain nature that [[Obsidian]] provides.
# The Stages

Before we dive deep into the technicalities of the process. Its best to first give an overview of the different stages and then we can highlight what the issues were and how they were overcome.


The workflow can be seen below in the following #excalidraw diagram:

![[../../../Excalidraw/Obsidian Blog Workflow.excalidraw|Obsidian Blog Workflow.excalidraw]]

## Stage 1: [[Obsidian]]

In this stage you make notes as usual and write your blog posts. The way in which I do this is by having a dedicated folder in my Obsidian Vault called `Blog`. This is where all my blog posts are written. I regularly push these notes to the `Gibbiverse Content` repository. I can write offline on my travels and grab inspiration from the Obsidian Vault in a quick glance. 

Typically I work out of a branch per blog as this allows me to run [[../../../Resources/Version Control/GitHub/GitHub Actions|GitHub Actions]] specifically [[MegaLinter]] which catches out formatting errors and spelling errors. I can also run a [[GitHub Agents]]
# Stage 2: [[../../../Resources/Version Control/GitHub/GitHub|GitHub]] [[Pull Request]]

## Stage 3: [[Hugo]] [[../../../Resources/Version Control/GitHub/GitHub Actions|GitHub Actions]]


## Updating Submodules

Once im comfortable with the blog post, I then move to the [[Hugo]] repository. In this repository I update the `Blog Posts` submodule by first creating a new branch and then pulling in the latest changes from the `Blog Posts` repository.  During this time I can see what the Blog would like by running the following command in the root of the repository

```bash
hugo server -D
```

Once me and the [[CICD]]  system ([[MegaLinter]] is used to check for linting across the project space) are happy with the results, I go on to merge this back into main.

## Committing to a Release

The workflow Im using for releasing the blog is the one provided by Hugo and uses GitHub Actions alongside GitHub Pages to create both a [[../../../Resources/Version Control/GitHub/GitHub|GitHub]] release alongside a published website. This workflow only runs on a tagged commit. As such I am in full control of when the next Blog post is published.
