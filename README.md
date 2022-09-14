En enkel halvsentralisert chatteapplikasjon for å kommunisere på et lokalnett.
Klienter vil sende serveren en registreringsmelding som inneholder brukernavn, ip adresse og port. Når man får en godkjenning fra serveren er man registrert og kan begynne å sende meldinger til andre klienter.
Serveren fungerer litt som en DNS server. Klienter kan sende en lookup melding for et brukernavn til serveren. Hvis serveren har cachet den forespurte brukeren sender den ip og port tilbake til klienten. Klienten kan så sende meldinger til andre klienter.

## Bruk
Server: ```cargo run --bin server```
Klient: ```cargo run --bin client```

## Problemer med løsningen 
- Kan spare plass ved å lage en egen serialisering
- Klienten blokkerer mens den venter på lookup response
- Sjekker ikke bruker input (navn kan være med whitespaces ol.)
- Hvis to med samme brukernavn kobler seg til vil den siste kaste ut den første
- Bruker for øyeblikket bare localhost
- Klienter blir aldri kastet ut hos serveren. Ideelt burde klienter sende heartbeats til serveren med gjevne mellomrom for å gi beskjed om at de fortsatt er aktive.
- Bruker bare ipv4 adresser (nettet jeg testet på hadde ikke støtte for ipv6)
