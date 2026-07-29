#!/usr/bin/env bash
# Block feature implementation when its Roadmap issue is too large or unestimated.
set -euo pipefail

readonly project_number=2
readonly project_owner="EwertonDCSilv"
readonly maximum_points=8

issue_number=""
branch_name="${GITHUB_HEAD_REF:-}"
if [[ -z "${branch_name}" ]]; then
  branch_name="$(git branch --show-current 2>/dev/null || true)"
fi

usage() {
  printf '%s\n' \
    "uso: scripts/check-agent-story-points.sh [--issue N] [--branch NAME]" \
    "" \
    "Sem --issue, o número é extraído de feature/<issue>-<descrição>." \
    "Branches que não são de feature são ignoradas."
}

while (($# != 0)); do
  case "$1" in
    --issue)
      if (($# < 2)); then
        printf '%s\n' "guard rail: --issue exige um número." >&2
        exit 2
      fi
      issue_number="$2"
      shift 2
      ;;
    --branch)
      if (($# < 2)); then
        printf '%s\n' "guard rail: --branch exige um nome." >&2
        exit 2
      fi
      branch_name="$2"
      shift 2
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      printf 'guard rail: opção desconhecida: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -n "${issue_number}" && ! "${issue_number}" =~ ^[1-9][0-9]*$ ]]; then
  printf 'guard rail: número de issue inválido: %s\n' "${issue_number}" >&2
  exit 2
fi

branch_issue=""
if [[ "${branch_name}" == feature/* ]]; then
  if [[ "${branch_name}" =~ ^feature/([1-9][0-9]*)-[a-z0-9]+([a-z0-9-]*[a-z0-9])?$ ]]; then
    branch_issue="${BASH_REMATCH[1]}"
  else
    printf '%s\n' \
      "guard rail: branch de feature inválida: ${branch_name}" \
      "Use feature/<issue-number>-<semantic-description>." >&2
    exit 1
  fi
elif [[ -z "${issue_number}" ]]; then
  printf 'guard rail: branch não é de feature; verificação ignorada (%s).\n' \
    "${branch_name:-detached HEAD}"
  exit 0
fi

if [[ -z "${issue_number}" ]]; then
  issue_number="${branch_issue}"
elif [[ -n "${branch_issue}" && "${issue_number}" != "${branch_issue}" ]]; then
  printf 'guard rail: --issue #%s não corresponde à issue #%s da branch %s.\n' \
    "${issue_number}" "${branch_issue}" "${branch_name}" >&2
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  printf '%s\n' \
    "guard rail: gh não está disponível; não foi possível consultar o Roadmap." \
    "Instale e autentique o GitHub CLI antes de implementar a feature." >&2
  exit 1
fi

query=".items[] \
| select(.content.number == ${issue_number}) \
| [(.[\"story points\"] // \"UNESTIMATED\"), \
   ((.labels // []) | join(\",\")), \
   .title, \
   .content.url] \
| @tsv"

set +e
roadmap_row="$(
  gh project item-list "${project_number}" \
    --owner "${project_owner}" \
    --limit 200 \
    --format json \
    --jq "${query}" 2>&1
)"
gh_status=$?
set -e

if ((gh_status != 0)); then
  printf '%s\n' \
    "guard rail: não foi possível consultar o Roadmap para a issue #${issue_number}." \
    "Confirme a autenticação do gh e a permissão de leitura do Projects v2." >&2
  exit 1
fi

if [[ -z "${roadmap_row}" ]]; then
  printf '%s\n' \
    "guard rail: issue #${issue_number} não está no clojure-compiler Roadmap." \
    "Cadastre e estime a issue antes de implementar." >&2
  exit 1
fi

if [[ "${roadmap_row}" == *$'\n'* ]]; then
  printf 'guard rail: issue #%s aparece mais de uma vez no Roadmap.\n' \
    "${issue_number}" >&2
  exit 1
fi

IFS=$'\t' read -r story_points labels title issue_url <<<"${roadmap_row}"

if [[ "${story_points}" == "UNESTIMATED" || -z "${story_points}" ]]; then
  printf '%s\n' \
    "guard rail: issue #${issue_number} não possui Story points." \
    "Estime a issue no Roadmap antes de implementar." >&2
  exit 1
fi

if [[ ! "${story_points}" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
  printf 'guard rail: Story points inválidos na issue #%s: %s.\n' \
    "${issue_number}" "${story_points}" >&2
  exit 1
fi

if [[ ",${labels}," == *",epic,"* || "${title}" == Epic:* ]]; then
  printf '%s\n' \
    "guard rail: épicos e roll-ups não podem ser implementados diretamente (#${issue_number})." \
    "Refine o épico em tasks independentes de até ${maximum_points} story points." >&2
  exit 1
fi

if awk -v points="${story_points}" -v maximum="${maximum_points}" \
  'BEGIN { exit !(points > maximum) }'; then
  printf '%s\n' \
    "guard rail: issue #${issue_number} possui ${story_points} story points; implementação bloqueada." \
    "Refine o escopo e, de preferência, crie sub-issues independentes de até ${maximum_points} pontos." \
    "Reestime as tasks antes de abrir uma branch de implementação." >&2
  exit 1
fi

printf 'guard rail: issue #%s aprovada com %s story points (%s).\n' \
  "${issue_number}" "${story_points}" "${issue_url}"
