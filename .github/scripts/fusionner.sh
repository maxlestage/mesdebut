#!/usr/bin/env bash
# Fusionne dans master chaque PR dont toutes les vérifications sont vertes.
#
# Lancé par le workflow fusion.yml à la fin de chaque vérification. Une PR est
# fusionnée (en squash, comme les précédentes) quand :
#   - elle vise master et n'est pas un brouillon ;
#   - sa branche ne commence pas par wip/ (pour garder une PR ouverte exprès) ;
#   - GitHub la dit fusionnable (pas de conflit) ;
#   - toutes les vérifications de son dernier commit sont terminées, et aucune
#     n'a échoué.
# Une vérification encore en cours fait simplement attendre : la suivante qui
# se termine relance ce script.
#
# Besoin de GH_TOKEN et GH_REPO (owner/nom), et de gh.
set -euo pipefail

# Le job de ce workflow : il ne s'attend pas lui-même.
MOI="${FUSION_JOB:-fusionner}"

# L'état de fusion d'une PR ; GitHub le calcule après chaque commit, on lui
# laisse quelques secondes.
fusionnable() {
  local etat
  for _ in 1 2 3 4 5 6; do
    etat="$(gh pr view "$1" --json mergeable --jq .mergeable)"
    [ "$etat" != UNKNOWN ] && break
    sleep 10
  done
  echo "$etat"
}

prs="$(gh pr list --base master --state open --limit 50 \
  --json number,headRefName,headRefOid,isDraft \
  --jq '.[] | [.number, .headRefName, .headRefOid, .isDraft] | @tsv')"

[ -n "$prs" ] || { echo "Aucune PR ouverte vers master."; exit 0; }

refus=0

while IFS=$'\t' read -r numero branche sha brouillon; do
  if [ "$brouillon" = true ]; then
    echo "#$numero : brouillon, pas encore."
    continue
  fi
  case "$branche" in
    wip/*) echo "#$numero : branche $branche, jamais fusionnée automatiquement."; continue ;;
  esac

  etats="$(gh api "repos/$GH_REPO/commits/$sha/check-runs?per_page=100" \
    --jq ".check_runs[] | select(.name != \"$MOI\") | .name + \"	\" + .status + \"	\" + (.conclusion // \"\")")"
  if [ -z "$etats" ]; then
    echo "#$numero : aucune vérification sur $sha, on attend."
    continue
  fi
  if echo "$etats" | awk -F'\t' '$2 != "completed"' | grep -q .; then
    echo "#$numero : vérifications en cours :"
    echo "$etats" | awk -F'\t' '$2 != "completed" { print "  - " $1 }'
    continue
  fi
  rouges="$(echo "$etats" | awk -F'\t' '$3 != "success" && $3 != "neutral" && $3 != "skipped" { print "  - " $1 " (" $3 ")" }')"
  if [ -n "$rouges" ]; then
    echo "#$numero : pas fusionnée, vérification en échec :"
    echo "$rouges"
    continue
  fi

  etat="$(fusionnable "$numero")"
  if [ "$etat" != MERGEABLE ]; then
    echo "#$numero : pas fusionnable ($etat)."
    continue
  fi

  echo "#$numero : tout est vert, fusion dans master."
  # --match-head-commit : un commit poussé entre-temps annule la fusion
  if ! gh pr merge "$numero" --squash --match-head-commit "$sha"; then
    # par exemple un commit poussé entre-temps (--match-head-commit) : la
    # vérification suivante relancera la fusion
    echo "::warning::#$numero : GitHub a refusé la fusion automatique."
    refus=$((refus + 1))
  fi
done <<< "$prs"

exit $((refus > 0))
