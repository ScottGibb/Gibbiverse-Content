---
tags:
  - blog-post
  - blogging
date: 2025-10-15
title: "Blogging: Obsidian to Hugo Workflow"
draft: false
---
# Summary

This is my first blog post and im going to talk about how to make an easy workflow between Obsidian and [[Hugo]]. I love using [[Obsidian]] and I love taking Notes, now its time to share them with the rest of the Internet. The approach I take is a three stage approach, which is outlined in the diagram below. We will walk our way through this workflow in this blog and outline the tools and the stages as we go through.

# What tools will you need?

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

![[Obsidian Blog Workflow.excalidraw.svg]]

## Stage 1: Writing your Blog

In this stage you make notes as usual and write your blog posts. The way in which I do this is by having a dedicated folder in my Obsidian Vault called `Blog`. This is where all my blog posts are written. I regularly push these notes to the `Gibbiverse Content` repository. I can write offline on my travels and grab inspiration from the Obsidian Vault in a quick glance.

Typically I work out of a branch per blog as this allows me to run [[../../../Resources/Version Control/GitHub/GitHub Actions|GitHub Actions]] specifically [[MegaLinter]] which catches out formatting errors and spelling errors. I can also run  [[GitHub Agents]] to check over my work and improve my writing style.

At this stage, the most of the work is being done as its focused on creating the blog itself and the content alongside some of the formatting.

Once i'm happy with the content and [[MegaLinter]] has performed its fixes. I merge the pull request into main and head on to Stage 2, where I work on the [[Hugo]] side in the [Gibbiverse](https://github.com/ScottGibb/Gibbiverse).

## Stage 2: Seeing the Final Content

In Stage 2 we create a new branch in the [Gibbiverse](https://github.com/ScottGibb/Gibbiverse) repository, where we can update the content submodule to point to the latest commit in the `Gibbiverse Content` repository.

When we are in this branch we can then work on the [[Hugo]] side of things. We can see what the website looks like locally, by running the [[../../../Resources/Languages/Python|Python]] script in the [Gibbiverse Link Fixer](https://github.com/ScottGibb/Gibbiverse-Link-Fixer). This will at present (15-10-2025) replace all the links in the active file location with the correct links for the website. This is important as [[Hugo]] will not be able to render the links correctly if they are pointing to my private knowledge base. However this same script is called in the [[../../../Resources/Version Control/GitHub/GitHub Actions|GitHub Actions]] workflow when the site is built and deployed.

Once the verification of the website is done and we have made all the tweaks to the [[Hugo]] portion off things we then move on Stage 3.

## Stage 3: Releasing your Blog

In Stage 3 we are ready to post our blog to the wider world through [[GitHub Pages]].  We submit a pull request to merge our changes into main. This ensures that our changes are correctly linted through [[MegaLinter]] and the code is up to date via [[Dependabot]].

When we are ready to run the release we then simple tag the commit we want to release and send it to [[../../../Resources/Version Control/GitHub/GitHub|GitHub]]. [[../../../Resources/Version Control/GitHub/GitHub Actions|GitHub Actions]] handles the rest and creates a Release on GitHub and then updates the web site accordingly.

# The Quirks and Issues

There are a few quirks and issues that I have found along the way. These are outlined below:

- [[Obsidian]] uses Wikilinks primarily and these arent supported by [[Hugo]]. This was a relatively easy fix as [[Obsidian]] now supports normal Markdown links. So I just need to ensure that I use these in my blog posts.

- Some of my links get replaced with external links. These links are held as key value pairs inside a [[yaml]] file. Within the `Gibbiverse Content` repo.

- Having the blogs reside inside my Vault often means I have a lot of internal links in my posts and these need to be removed before publishing. This is done using the [[../../../Resources/Languages/Python|Python]] script in the [Gibbiverse Link Fixer](https://github.com/ScottGibb/Gibbiverse-Link-Fixer) a small [[../../../Resources/Languages/Python|Python]] project that can be used to counteract it. It also goes through the code and adds frontmatter tags. These are also held within the `Gibbiverse Content` repo.

# Closing Remarks

Hopefully this short blog gave an overview of the Blogging workflow and hopefully this workflow continues to work for many future Blogs to come. If you have any questions please reach out on [[../../../Resources/Version Control/GitHub/GitHub|GitHub]] and post an issue or if you have improvements to the blogs please submit a [[pull request]].

## Repositories

- [Gibbiverse](https://github.com/ScottGibb/Gibbiverse)
- [Gibbiverse Content](https://github.com/ScottGibb/Gibbiverse-Content)
- [Gibbiverse Link Fixer](https://github.com/ScottGibb/Gibbiverse-Link-Fixer)
