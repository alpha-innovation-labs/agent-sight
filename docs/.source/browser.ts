// @ts-nocheck
import { browser } from 'fumadocs-mdx/runtime/browser';
import type * as Config from '../source.config';

const create = browser<typeof Config, import("fumadocs-mdx/runtime/types").InternalTypeConfig & {
  DocData: {
  }
}>();
const browserCollections = {
  docs: create.doc("docs", {"index.mdx": () => import("../content/docs/index.mdx?collection=docs"), "overview.mdx": () => import("../content/docs/overview.mdx?collection=docs"), "reference.mdx": () => import("../content/docs/reference.mdx?collection=docs"), "getting-started/commands.mdx": () => import("../content/docs/getting-started/commands.mdx?collection=docs"), "getting-started/installation.mdx": () => import("../content/docs/getting-started/installation.mdx?collection=docs"), }),
};
export default browserCollections;