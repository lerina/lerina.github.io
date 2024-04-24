<main>

# Cahier des charges: périmètre fonctionnel

<h2>Dates importantes</h2>

- Date d’émission de la consultation: 27 Mars 2024
- Date et heure limites de remise des offres: 17 Mai 2024 à 12H00 (heure locale)
- Date et heure limites de livraison de la prestation<sup>1</sup>: 1er septembre 2024

<sup>1</sup> *prestation*: Le Site, la Maintenance et l'Hebergement

## Introduction

## L'équipe technique



## Cahier des charges

- Demande Explicite

1. Noyau
2. Médias
3. Mobilité
4. Actualités
5. Alerte
6. Agenda (Event/calendar)
7. Partage sur les réseaux sociaux
8. Moteur de recherche
9. Générateur de formulaire
10. Offres d’emploi
11. Recherche de logements
12. Chat
13. Témoignages

- Spécification implicite

1. 


### Noyau (l’admin du site)

Les administrateurs de la plate-forme devront pouvoir :

- Ajouter, modifier ou supprimer des pages, rubriques, sous-rubriques… ;
- Ajouter, modifier ou supprimer des éléments composant le zoning (header, footer, menu principal et secondaire, etc) ;
- Programmer des publications (date de mise en ligne, date de fin de publication…);
- Mettre à jour l’arrière-plan graphique du site internet ;
- Gérer l’arborescence ;
- Gérer les droits.

NOTE: zoning = wireframe

#### voir plugin et documentation
 
- [Administrer votre site](https://fr.wordpress.org/support/article/administering-your-blog/){target="_blank"}

### Médias

Le MS devra permettre de mettre en ligne :

- Des vidéos, soit directement sur le site, soit via un hébergeur (type Youtube) ;
- Des sons, soit directement sur le site, soit via un hébergeur ;
- Des photos qui pourront : - Être redimensionnées automatiquement ; - Faire l’objet de quelques retouches simples (recadrage).

#### voir plugin et documentation

- [Médiathèque](https://wordpress.com/fr/support/media/){target="_blank"}
- voir aussi [Gérer les Médias WordPress](https://deligraph.com/tuto-wordpress/gerer-les-medias-wordpress/){target="_blank"}

### Mobilité

Le site devra impérativement pouvoir être visionné en format adapté selon le principe du ~responsable~ responsive web design,

- sur mobile,
- tablette,
- ordinateur.

Lors de la création graphique, de la conception et du développement, le prestataire devra donner la possibilité à l’AaDTM de visionner le site sur ces 3 supports.

#### voir plugin et documentation

- [Creating Mobile Responsive Websites](https://wordpress.com/go/web-design/mobile-responsive-design/){target="_blank"}

### Actualités

L’AaDTM souhaite pouvoir rédiger et mettre en avant des actualités.

- L’utilisateur du site devra pouvoir trouver et filtrer facilement les informations par thématique (à définir ultérieurement) ou à l’aide d’un moteur de recherche.
- L’ordre de priorité des actualités sera défini par l’administrateur.
- Certaines actualités devront pouvoir être positionnées, au choix de l’administrateur, en page d’accueil du site.

#### voir plugin et documentation

- [WP Latest Posts](https://fr.wordpress.org/plugins/wp-latest-posts/){target="_blank"}

### Alerte

Les gestionnaires de la plate-forme devront pouvoir alerter les usagers de manière immédiate dans le cas d’événements importants réclamant une alerte.

Celle-ci devra être intégrée dans une fenêtre pop-up que l’internaute pourra fermer, soit par un bandeau défilant sur la page.

#### voir plugin et documentation

- [](){}{target="_blank"}

### Agenda (Event/calendar)

L’AaDTM souhaite mettre en place un agenda qui répertorie et diffuse l’ensemble des évènements.

- L’utilisateur du site devra pouvoir trouver et filtrer facilement les informations par date, par thématique (à définir) ou à l’aide d’un moteur de recherche.
- L’agenda sera classé par défaut par ordre chronologique, mais l’administrateur devra pouvoir prioriser les informations de façon à en faire apparaître certaines de manière évidente sur la page dédiée à l’agenda ou sur la page d’accueil du site.
- L’agenda devra être contributif : certains partenaires de l’AaDTM devront pouvoir proposer un nouvel évènement sur l’agenda ou modifier un évènement.
    * Ils pourront le faire de manière simple,
    * sans nécessité de login/mdp,
    * via un formulaire depuis le front office du site.
- Les informations qui seront proposées par les partenaires feront l’objet d’une modération obligatoire par les administrateurs du site.

#### voir plugin et documentation

- [](){}{target="_blank"}

### Partage sur les réseaux sociaux

L’AaDTM souhaite, depuis chaque page, des “icon Partage” sur les réseaux sociaux.
Annuaire

L’AaDTM souhaite mettre en place un annuaire de l’ensemble des associations, services publics et autres types de services sur le plan territorial. L’utilisateur devra pouvoir trouver et filtrer facilement les informations

- par thématique (à définir)
- par catégorie (à définir)
- ou à l’aide d’un moteur de recherche.

L’annuaire sera classé par défaut par ordre alphabétique.

#### voir plugin et documentation

- [](){}{target="_blank"}

### Moteur de recherche

Etant donné que la majorité des utilisateurs procéderont à une recherche sur le site, pour trouver une information, la’AaDTM souhaite mettre en place un moteur de recherche performant.
Autant que possible,

- le moteur de recherche devra permettre d’indexer la totalité des contenus (et méta- données associées) des sites y compris les contenus des fichiers (documents PDF et autres formats) et le co-marquage.
- Les caractéristiques attendues de la recherche sont :
    * recherche en texte intégral,
    * nombre de mots illimité dans une requête.
    * L’affichage des résultats se fera par ordre de pertinence avec mise en évidence des mots recherchés.
    * La recherche pourra être contextualisée par rubrique.
    * Le champ de saisie pour lancer une requête devra être présent en permanence sur toutes les pages du site.
    * Le moteur de recherche devra interpréter toutes les formes des noms, adjectifs et verbes (masculin/féminin, singulier/pluriel, formes conjuguées, accents, majuscules/minuscules), opérations booléennes, proximité des mots, etc et permettra de tirer les résultats (par date, types de contenus rubriques, etc).

#### voir plugin et documentation

- [](){}{target="_blank"}

### Générateur de formulaire

- Chaque formulaire sera renvoyé vers le ou les destinataire(s) chargé(s) d’instruire les demandes via Microsoft 365 (l’AaDTM souhaite que les formulaires reçus par mail n’arrivent pas dans les spam).
- Chaque formulaire entrainera une fiche sommaire vers le ou les destinataire(s) chargé(s) d’instruire les demandes.
- Les informations seront inscrites dans une base de données et de répertoires en cas d’upload d’image(s) par exemple, accessible uniquement par les administrateurs du site.

Le soumissionnaire fera son affaire de toute notion de sécurité,

- anti- virus,
- anti-ransomware,
- anti- phishing,
- contrôle d’usurpation d’identité par MFA si nécessaire (en utilisant authenticator Microsoft).

Impression de pages et conversion PDF

L’ensemble des pages du site devra pouvoir être imprimé ou converti en PDF facilement, à l’aide d’une icône située sur chaque page du site.

#### voir plugin et documentation

- [](){}{target="_blank"}

### Offres d’emploi

Une page dédiée aux offres d’emploi sera proposée sur la plate-forme. Sur cette page,

- on retrouvera l’ensemble des offres en cours.
- Les offres d’emploi créées devront se cacher automatiquement à une date définie par l’administrateur.
- Les différentes rubriques au sein des offres d’emploi :
    * Date de parution
    * Date limite de remise des candidatures
    * Service
    * Type de contrat
    * Catégorie
    * Missions
    * Compétences et qualités requises
    * Modalités
    * Informations complémentaires
    * Contact Postuler en ligne

Les candidats auront la possibilité

- de répondre en ligne via un formulaire leur permettant de déposer un CV et lettre de motivation.
- Réception de ces éléments sur une adresse mail dédiée qui sera communiquée ultérieurement par l’AaDTM
- et aussi directement sur l’adresse mail de l’annonceur de l’offre d’emploi.
- Une réponse par mail sera automatiquement générée, accusant réception de leur candidature.

#### voir plugin et documentation

- [](){}{target="_blank"}

### Recherche de logements

L’AaDTM souhaite relayer et/ou créer des annonces d’offres de logements sur la plate-forme.

- Les annonces seront préalablement validées par une collaboration entre l’AaDTM et les agences de locations de l’île.
- Les candidats auront la possibilité de répondre en ligne via un formulaire
    * leur permettant de manifester leur intérêt avec une réception du mail dans une adresse mail dédiée
    * et directement à l’adresse mail du partenaire à l’origine de l’annonce.

#### voir plugin et documentation

- [](){}{target="_blank"}

### Chat

L’AaDTM souhaite donner la possibilité aux utilisateurs de pouvoir engager des échanges avec les administrateurs à travers des réactions ou des questions sous forme d’un Chat. Il convient de préparer un système qui permettra la mise en place de ce Chat et aussi l’animation de ce Chat par l’AaDTM.

#### voir plugin et documentation

- [](){}{target="_blank"}

### Témoignages

Une place de choix devra être réservée aux témoignages de résidents et de partenaires de l’AaDTM. Les témoignages pourront prendre la forme

- d’une interview texte
- ou d’interviews vidéos avec un contenu qui sera déterminé et géré par l’AaDTM.

Il convient donc d’envisager sur le plan technique les moyens de diffusion de ces témoignages et sur le plan de la visibilité, de déterminer un positionnement approprié sur la plate-forme.

#### voir plugin et documentation

- [](){}{target="_blank"}

</main>
