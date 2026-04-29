#!/usr/bin/env bash
# Run LLM tool selection tests for a batch of providers
#
# Usage:
#   ./run_batch.sh --batch 1          # Run providers 1-50
#   ./run_batch.sh --batch 2          # Run providers 51-100
#   ./run_batch.sh --batch 1 --scale medium
#   ./run_batch.sh --providers github,slack,notion  # Run specific providers

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
# Portal is the project root (where Cargo.toml is)
ROOT_DIR="${TEST_DIR}/../.."  # goes from tests/llm_tool_selection to portal

# Default values
BATCH=""
PROVIDERS=""
SCALE="medium"
ROUNDS="1"
MODEL="minimax:latest"
DELAY_MS="100"

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --batch)
      BATCH="$2"
      shift 2
      ;;
    --providers)
      PROVIDERS="$2"
      shift 2
      ;;
    --scale)
      SCALE="$2"
      shift 2
      ;;
    --rounds)
      ROUNDS="$2"
      shift 2
      ;;
    --model)
      MODEL="$2"
      shift 2
      ;;
    --delay-ms)
      DELAY_MS="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Build providers list
PROVIDER_LIST=""

if [[ -n "$BATCH" ]]; then
  # Provider batch mapping (providers sorted by tool count, grouped by 50)
  case $BATCH in
    1)
      PROVIDER_LIST="toggl,timeular,google_workspace,posthog,salesforce_more,freshservice,servicenow,ordinal,grist,pleo,miro-scim,lastpass,oracle-cloud-identity,alibaba,autobot,bamboohr-basic,holded,sage-intacct-oauth,kintone-user-api,quora,perimeter81,twenty-crm-self-hosted,private-api-basic,reapit,box,ticktick,circleback-mcp,fillout,statista,autodesk,reply-io,figma-scim,looker-oauth,gusto,redtail-crm-sandbox,scrapedo,itglue,google-maps,freshsales,chatfuel,microsoft-oauth2-cc,stay-ai,roller,tally,bamboohr,sentry-oauth,loom-scim,openai,keap"
      ;;
    2)
      PROVIDER_LIST="recharge,kontent,stripe,callrail,streak,accelo,algolia,google-calendar,brex,microsoft_graph,workday-refresh-token,github,slack,notion,jira,gitlab,linear,asana,trello,hubspot,zendesk,discourse,wordpress,shopify,quickbooks,xero,discord,zoom,mattermost,bitbucket,jira-service-desk,confluence,google-drive,dropbox,onedrive,box-platform,slack-discovery,zoom-info,linkedin,twitter,facebook,instagram,tiktok,youtube"
      ;;
    *)
      echo "Unknown batch number. Batches available: 1, 2"
      exit 1
      ;;
  esac
elif [[ -n "$PROVIDERS" ]]; then
  PROVIDER_LIST="$PROVIDERS"
else
  echo "Error: Either --batch or --providers must be specified"
  exit 1
fi

# Check API key
if [[ -z "${MINIMAX_API_KEY:-}" ]]; then
  echo "ERROR: MINIMAX_API_KEY environment variable not set"
  exit 1
fi

# Run tests
echo "=========================================="
echo "Running LLM Tool Selection Tests"
echo "=========================================="
echo "Batch/Providers: ${PROVIDER_LIST:0:80}..."
echo "Scale: $SCALE"
echo "Rounds: $ROUNDS"
echo "Model: $MODEL"
echo "Delay between requests: ${DELAY_MS}ms"
echo ""

# Run from portal directory
cd "${ROOT_DIR}"

RUST_LOG=info cargo run -p llm_tool_selection_test -- \
  --scale "${SCALE}" \
  --rounds "${ROUNDS}" \
  --model "${MODEL}" \
  --registry tools/registry \
  --queries tests/llm_tool_selection/queries \
  --llm-url http://localhost:4000 \
  --llm-api-key "${MINIMAX_API_KEY}" \
  --providers "${PROVIDER_LIST}" \
  --delay-ms "${DELAY_MS}" \
  --output "/tmp/test_results_batch_${BATCH:-custom}"

echo ""
echo "Tests completed!"
echo "Results saved to /tmp/test_results_batch_${BATCH:-custom}/"
