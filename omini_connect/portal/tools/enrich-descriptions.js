#!/usr/bin/env node
/**
 * Tool Description Enricher using MiniMax API
 *
 * Reads all YAML files in tools/registry/, calls MiniMax API to generate improved
 * LLM-optimized descriptions for each tool, and writes back the enriched YAML files.
 */

const fs = require('fs');
const path = require('path');
const https = require('https');
const { URL } = require('url');

const REGISTRY_DIR = '/Users/shileipeng/Documents/mygithub/OminiConnect/omini_connect/portal/tools/registry';
const CONCURRENCY = parseInt(process.argv.find(a => a.startsWith('--concurrency='))?.split('=')[1] || '8');
const DRY_RUN = process.argv.includes('--dry-run');
const MINIMAX_API_KEY = process.env.MINIMAX_API_KEY;

if (!MINIMAX_API_KEY) {
  console.error('Error: MINIMAX_API_KEY environment variable not set');
  process.exit(1);
}

/**
 * Call MiniMax API with a prompt, return the response text
 */
async function miniMaxComplete(prompt, groupId) {
  return new Promise((resolve, reject) => {
    const body = JSON.stringify({
      model: 'MiniMax-Text-01',
      messages: [{
        role: 'user',
        content: prompt
      }],
      temperature: 0.7,
      max_tokens: 512
    });

    const url = new URL(`https://api.minimaxi.chat/v1/text/chatcompletion_v2?GroupId=${groupId}`);

    const options = {
      hostname: url.hostname,
      port: 443,
      path: url.pathname + url.search,
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${MINIMAX_API_KEY}`,
        'Content-Length': Buffer.byteLength(body)
      }
    };

    const req = https.request(options, (res) => {
      const chunks = [];
      res.on('data', chunk => chunks.push(chunk));
      res.on('end', () => {
        const data = Buffer.concat(chunks).toString();
        if (res.statusCode !== 200) {
          reject(new Error(`MiniMax API error ${res.statusCode}: ${data}`));
          return;
        }
        try {
          const json = JSON.parse(data);
          const choices = json.choices || [];
          if (choices.length > 0) {
            resolve(choices[0].message?.content || '');
          } else {
            resolve('');
          }
        } catch (e) {
          reject(new Error(`Failed to parse MiniMax response: ${data}`));
        }
      });
    });

    req.on('error', reject);
    req.write(body);
    req.end();
  });
}

/**
 * Generate improved description using MiniMax
 */
async function generateDescription(tool, provider) {
  const prompt = `You are an API documentation expert. Generate a clear, concise LLM-optimized description for this API tool. The description should help an AI agent understand exactly what this tool does and when to use it.

Guidelines:
- 2-3 sentences maximum
- Start with an action verb (List, Get, Create, Update, Delete, Search, etc.)
- Mention what data it returns or what it creates/modifies
- Be specific about the entity type (not generic)
- Do NOT repeat the endpoint path in the description
- Do NOT use quotes or markdown formatting in the response

Tool Name: ${tool.name}
Endpoint: ${tool.method} ${tool.endpoint}
Current Description: ${tool.description || 'No description'}
Tags: ${(tool.tags || []).join(', ') || 'none'}
Required Scopes: ${(tool.scopes || []).join(', ') || 'none'}

Respond with ONLY the improved description text, no preamble.`;

  try {
    const result = await miniMaxComplete(prompt, provider);
    return result.trim().replace(/^["']|["']$/g, '').replace(/\n+/g, ' ').replace(/\s+/g, ' ').trim();
  } catch (e) {
    console.error(`  LLM error for ${tool.slug}: ${e.message}`);
    return null;
  }
}

/**
 * Process a single YAML file
 */
async function processYamlFile(filepath) {
  const content = fs.readFileSync(filepath, 'utf8');
  const provider = path.basename(filepath, '.yaml');
  const filename = path.basename(filepath);

  // Check if it has tool entries
  if (!content.includes('- slug:')) {
    return { skipped: true };
  }

  // Split into parts - header and tools
  const lines = content.split('\n');
  const headerLines = [];
  const toolLines = [];
  let inHeader = true;
  let inTool = false;
  let currentToolStart = -1;
  let currentToolLines = [];

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (inHeader && line.match(/^- slug:/)) {
      // Start of first tool
      inHeader = false;
      currentToolStart = i;
      currentToolLines = [line];
      inTool = true;
    } else if (inHeader) {
      headerLines.push(line);
    } else if (inTool) {
      if (line.match(/^- slug:/) && i !== currentToolStart) {
        // New tool starting, process the completed one
        const toolContent = currentToolLines.join('\n');
        const descMatch = toolContent.match(/^\s+description:\s*\|\s*\n((?:\s{2,}[^\n]+\n)*)/m);

        if (descMatch) {
          const slugMatch = toolContent.match(/^\s+slug:\s*(\S+)/);
          const nameMatch = toolContent.match(/name:\s*([^\n]+)/);
          const endpointMatch = toolContent.match(/endpoint:\s*([^\n]+)/);
          const methodMatch = toolContent.match(/method:\s*([^\n]+)/);
          const scopesMatch = toolContent.match(/scopes:\s*\n((?:\s{2,}-\s[^\n]+\n)*)/m);
          const tagsMatch = toolContent.match(/tags:\s*\n((?:\s{2,}-\s[^\n]+\n)*)/m);

          const slug = slugMatch ? slugMatch[1] : '';
          const name = nameMatch ? nameMatch[1].trim() : slug;
          const endpoint = endpointMatch ? endpointMatch[1].trim() : '';
          const method = methodMatch ? methodMatch[1].trim() : 'GET';
          const scopes = scopesMatch ? scopesMatch[1].split('\n').map(l => l.replace(/^\s+-\s+/, '').trim()).filter(Boolean) : [];
          const tags = tagsMatch ? tagsMatch[1].split('\n').map(l => l.replace(/^\s+-\s+/, '').trim()).filter(Boolean) : [];

          const originalDesc = descMatch[1].split('\n').map(l => l.replace(/^\s+/, '')).join(' ').trim();

          // Call LLM to generate improved description
          const tool = { slug, name, endpoint, method, description: originalDesc, scopes, tags };
          const improvedDesc = await generateDescription(tool, provider);

          if (improvedDesc && improvedDesc.length > 20 && improvedDesc !== originalDesc) {
            const indent = descMatch[0].match(/^(\s*)/)[1] || '    ';
            const newDescBlock = improvedDesc.split('\n').map(l => indent + '    ' + l).join('\n') + '\n';
            const newToolContent = toolContent.replace(
              /^\s+description:\s*\|\s*\n((?:\s{2,}[^\n]+\n)*)/m,
              '  description: |\n' + newDescBlock
            );
            toolLines.push(newToolContent);
            console.log(`  [${filename}] ${slug}: "${improvedDesc.substring(0, 80)}..."`);
          } else {
            toolLines.push(toolContent);
            if (improvedDesc) {
              console.log(`  [${filename}] ${slug}: kept (no improvement needed)`);
            } else {
              console.log(`  [${filename}] ${slug}: kept (LLM failed)`);
            }
          }
        } else {
          toolLines.push(toolContent);
        }

        currentToolStart = i;
        currentToolLines = [line];
      } else if (line.match(/^---/) && currentToolLines.length > 0) {
        // Document separator
        const toolContent = currentToolLines.join('\n');
        toolLines.push(toolContent);
        toolLines.push(line);
        currentToolStart = -1;
        currentToolLines = [];
        inTool = false;
      } else {
        currentToolLines.push(line);
      }
    } else if (line.match(/^---/)) {
      inTool = true;
      currentToolStart = i;
      currentToolLines = [line];
    } else {
      toolLines.push(line);
    }
  }

  // Process last tool if exists
  if (currentToolLines.length > 0) {
    const toolContent = currentToolLines.join('\n');
    const descMatch = toolContent.match(/^\s+description:\s*\|\s*\n((?:\s{2,}[^\n]+\n)*)/m);

    if (descMatch) {
      const slugMatch = toolContent.match(/^\s+slug:\s*(\S+)/);
      const nameMatch = toolContent.match(/name:\s*([^\n]+)/);
      const endpointMatch = toolContent.match(/endpoint:\s*([^\n]+)/);
      const methodMatch = toolContent.match(/method:\s*([^\n]+)/);
      const scopesMatch = toolContent.match(/scopes:\s*\n((?:\s{2,}-\s[^\n]+\n)*)/m);
      const tagsMatch = toolContent.match(/tags:\s*\n((?:\s{2,}-\s[^\n]+\n)*)/m);

      const slug = slugMatch ? slugMatch[1] : '';
      const name = nameMatch ? nameMatch[1].trim() : slug;
      const endpoint = endpointMatch ? endpointMatch[1].trim() : '';
      const method = methodMatch ? methodMatch[1].trim() : 'GET';
      const scopes = scopesMatch ? scopesMatch[1].split('\n').map(l => l.replace(/^\s+-\s+/, '').trim()).filter(Boolean) : [];
      const tags = tagsMatch ? tagsMatch[1].split('\n').map(l => l.replace(/^\s+-\s+/, '').trim()).filter(Boolean) : [];

      const originalDesc = descMatch[1].split('\n').map(l => l.replace(/^\s+/, '')).join(' ').trim();

      const tool = { slug, name, endpoint, method, description: originalDesc, scopes, tags };
      const improvedDesc = await generateDescription(tool, provider);

      if (improvedDesc && improvedDesc.length > 20 && improvedDesc !== originalDesc) {
        const indent = descMatch[0].match(/^(\s*)/)[1] || '    ';
        const newDescBlock = improvedDesc.split('\n').map(l => indent + '    ' + l).join('\n') + '\n';
        const newToolContent = toolContent.replace(
          /^\s+description:\s*\|\s*\n((?:\s{2,}[^\n]+\n)*)/m,
          '  description: |\n' + newDescBlock
        );
        toolLines.push(newToolContent);
        console.log(`  [${filename}] ${slug}: "${improvedDesc.substring(0, 80)}..."`);
      } else {
        toolLines.push(toolContent);
        console.log(`  [${filename}] ${slug}: kept (no change)`);
      }
    } else {
      toolLines.push(toolContent);
    }
  }

  return {
    skipped: false,
    content: headerLines.join('\n') + '\n' + toolLines.join('\n')
  };
}

/**
 * Main function
 */
async function main() {
  console.log(`\n=== Tool Description Enricher (MiniMax) ===`);
  console.log(`  Registry: ${REGISTRY_DIR}`);
  console.log(`  Concurrency: ${CONCURRENCY}`);
  console.log(`  Dry run: ${DRY_RUN}\n`);

  const files = fs.readdirSync(REGISTRY_DIR)
    .filter(f => f.endsWith('.yaml'))
    .sort();

  console.log(`Found ${files.length} YAML files\n`);

  let completed = 0;
  let errors = 0;
  let skipped = 0;
  const queue = [...files];
  const running = [];

  async function processQueue() {
    while (queue.length > 0 && running.length < CONCURRENCY) {
      const file = queue.shift();
      const filepath = path.join(REGISTRY_DIR, file);

      const p = (async () => {
        try {
          const result = await processYamlFile(filepath);
          if (result.skipped) {
            skipped++;
          } else {
            if (!DRY_RUN) {
              fs.writeFileSync(filepath, result.content, 'utf8');
            }
            completed++;
          }
        } catch (e) {
          console.error(`Error on ${file}: ${e.message}`);
          errors++;
        }
      })();

      running.push(p);
      p.finally(() => {
        const idx = running.indexOf(p);
        if (idx >= 0) running.splice(idx, 1);
      });

      // Small delay between requests to respect rate limits
      if (running.length >= CONCURRENCY || queue.length === 0) {
        await Promise.race(running);
        await new Promise(r => setTimeout(r, 300));
      }
    }

    if (queue.length > 0) {
      await Promise.race(running);
      return processQueue();
    }

    if (running.length > 0) {
      await Promise.all(running);
    }
  }

  await processQueue();

  console.log(`\n=== Done ===`);
  console.log(`Completed: ${completed}, Skipped: ${skipped}, Errors: ${errors}`);
}

main().catch(console.error);
