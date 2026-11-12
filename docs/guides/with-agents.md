---
title: Working with agents
description: Let a coding or chat agent write, check and render reports with the renderreport skill.
order: 9
---

A coding agent can build a report from a conversation: it reads the component catalog, writes a
[render request](../../reference/render-request/), checks it and renders the PDF. The repository
ships a skill that describes this route.

## The skill

The skill is one file, [`skills/renderreport/SKILL.md`](https://github.com/casoon/renderreport/blob/main/skills/renderreport/SKILL.md).
It says when to use renderreport, the order of the CLI calls, how a report is structured
(title page, summary first, findings, appendix), how to use [templates and
data](../templates-and-data/) and [`renderreport from`](../from-files/), and the mistakes that
agents make most often.

To use it, copy the folder `skills/renderreport` into the skills directory of your agent, for
Claude Code `.claude/skills/` in a project or `~/.claude/skills/` for all projects. The agent
needs the `renderreport` binary on its `PATH` and nothing else.

## Why the loop is short

The agent does not need to know the components. Three commands carry it:

```sh
renderreport list-components                      # what exists
renderreport schema --component metric-strip      # the fields of one component
renderreport validate report.json                 # what is wrong, by component and field
```

`validate` names the component index, the field and the expected type, so the agent corrects
the request and runs it again until it reports `Request is valid`. Then `render` produces the
PDF, and `--format png --pages 1` gives a page image the agent can look at to check what
`validate` cannot see, such as a missing section or a bad page break.

## MCP

There is no MCP server yet. The CLI covers the same steps, and every agent that can run a shell
can use it. An MCP server (`list_components`, `component_schema`, `validate`, `render`) is
planned only if agents without a shell turn out to need it.
