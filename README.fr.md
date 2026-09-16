<!-- SPDX-FileCopyrightText: 2026 Libre AI contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->
<!-- Written for the retained Libre AI portfolio on 2026-09-14; earlier source documents and revisions retain their original licensing. -->

# Libre AI Database Policy Inspector

## Usage visé

Aider un développeur à examiner les changements liés aux bases de données et à comprendre les constats avant de décider quoi modifier. DB Inspector vise à fournir des preuves sur SQL, les règles d’accès et les migrations, sans devenir l’autorité durable sur les données d’un produit.

## Candidats existants et limites

Les anciennes sources contiennent un candidat d’inspection statique en ligne de commande pour des fichiers et règles liés à PostgreSQL. Ce candidat documentaire n’admet ni CLI installé, ni connexion à une base réelle, ni réparation automatique, ni garantie de correction des contrôles d’accès applicatifs.

## Frontières proposées

Une inspection retenue précise ses fichiers d’entrée, sa syntaxe prise en charge, la version des règles et ses limites. Les constats doivent distinguer les preuves observées des analyses non prises en charge ou indisponibles. L’inspection statique est distincte du comportement réel de la base. Toute future inspection connectée ou capacité d’écriture nécessite son propre périmètre explicite, ses permissions et sa qualification.

## Critères d’activation

Qualifier les règles statiques retenues avec des fichiers représentatifs, des constats déterministes et des cas explicites de refus ou de non-prise en charge. Vérifier une sortie utile dans un consommateur CLI propre et l’absence d’écriture ou de divulgation indue. L’admission d’un module statique ne nécessite pas d’intégration à une base réelle ; une intégration connectée demanderait ses propres preuves et ne bloque pas l’existence documentaire du dépôt.

[English](README.md)

## Navigation du portefeuille

Ces liens décrivent le portefeuille retenu visé. La disponibilité publique et l’accessibilité ne sont pas vérifiées pour ce candidat privé.

### Produits

- [Libre AI Work Supervision](https://github.com/libre-ai/ai-work-supervision)
- [Libre AI Model Policy](https://github.com/libre-ai/ai-model-policy)
- [Libre AI Practice Workbench](https://github.com/libre-ai/ai-practice-workbench)
- [Libre AI Learning Session Facilitation](https://github.com/libre-ai/learning-session-facilitation)
- [Libre AI Personal Knowledge Notebook](https://github.com/libre-ai/personal-knowledge-notebook)
- [Libre AI Information Feed Filter](https://github.com/libre-ai/information-feed-filter)
- [Libre AI Travel Itinerary Planner](https://github.com/libre-ai/travel-itinerary-planner)
- [Libre AI Public Vote Comparison](https://github.com/libre-ai/public-vote-comparison)

### Composants et outils

- [Libre AI Application Development Toolkit](https://github.com/libre-ai/application-development-toolkit)
- [Libre AI Schemas And Contracts](https://github.com/libre-ai/schemas-and-contracts)
- [Libre AI Collaborative Data Sync](https://github.com/libre-ai/collaborative-data-sync)
- [Libre AI Execution Continuity Evaluator](https://github.com/libre-ai/execution-continuity-evaluator)
- [Libre AI Execution Sandbox](https://github.com/libre-ai/execution-sandbox)
- [Libre AI Capability Authorization](https://github.com/libre-ai/capability-authorization)
- [Libre AI Organization Data Lifecycle](https://github.com/libre-ai/organization-data-lifecycle)
- [Libre AI Database Policy Inspector](https://github.com/libre-ai/database-policy-inspector)
- [Libre AI Artifact Verification](https://github.com/libre-ai/artifact-verification)

### Projet

- [Libre AI](https://github.com/libre-ai/.github)
- [Libre AI Project Website](https://github.com/libre-ai/project-website)
- [Libre AI Project Governance](https://github.com/libre-ai/project-governance)



---

## Source éditoriale revue

[Matière revue](https://github.com/libre-ai/database-policy-inspector/blob/06cd3dd1161ad7ee17f78039374f8b671babc24a/docs/portfolio-material.json)

SHA-256: `ad0fa587ede49eae7075629bd75923238b295136c27e5c48aa50ad873bef6294`
