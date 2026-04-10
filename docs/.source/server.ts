// @ts-nocheck
import * as __fd_glob_6 from "../content/docs/getting-started/installation.mdx?collection=docs"
import * as __fd_glob_5 from "../content/docs/getting-started/commands.mdx?collection=docs"
import * as __fd_glob_4 from "../content/docs/reference.mdx?collection=docs"
import * as __fd_glob_3 from "../content/docs/overview.mdx?collection=docs"
import * as __fd_glob_2 from "../content/docs/index.mdx?collection=docs"
import { default as __fd_glob_1 } from "../content/docs/getting-started/meta.json?collection=docs"
import { default as __fd_glob_0 } from "../content/docs/meta.json?collection=docs"
import { server } from 'fumadocs-mdx/runtime/server';
import type * as Config from '../source.config';

const create = server<typeof Config, import("fumadocs-mdx/runtime/types").InternalTypeConfig & {
  DocData: {
  }
}>({"doc":{"passthroughs":["extractedReferences"]}});

export const docs = await create.docs("docs", "content/docs", {"meta.json": __fd_glob_0, "getting-started/meta.json": __fd_glob_1, }, {"index.mdx": __fd_glob_2, "overview.mdx": __fd_glob_3, "reference.mdx": __fd_glob_4, "getting-started/commands.mdx": __fd_glob_5, "getting-started/installation.mdx": __fd_glob_6, });