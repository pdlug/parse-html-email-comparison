import { parseHTML } from "linkedom";

import * as fs from "fs";

type Message = {
  id: string;
  fromEmail: string;
  fromName: string | null;
  htmlBody: string;
};

function extractLinks(html: string): string[] {
  const { document } = parseHTML(html);
  const links: string[] = [];

  document.querySelectorAll("a").forEach((link) => {
    const target = link.getAttribute("href");
    if (target && target.startsWith("http")) {
      links.push(target);
    }
  });

  return links;
}

(async () => {
  const startTime = new Date().getTime();

  const messages: Message[] = JSON.parse(
    fs.readFileSync("../messages.json", "utf-8")
  );
  const allLinks: string[] = [];

  messages.forEach((message) => {
    if (message.htmlBody) {
      const links = extractLinks(message.htmlBody);
      if (links.length > 0) {
        allLinks.push(...links);
      }
    }
  });

  fs.writeFileSync("./links.txt", allLinks.join("\n"));

  const endTime = new Date().getTime();
  console.log(`time: ${endTime - startTime}ms`);
})();
