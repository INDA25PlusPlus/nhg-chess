nu ska vi schacka loss 💃

# bibblans flöde
1. spelaren matar ett coord. (i nuläget bit). *get_piece_at* svarar med om det finns en pjäs där; om ja, ger den vilken typ.
2. med *select_piece* kan spelaren välja en pjäs. inom den funktionen är *player_tracker* vilket ser till att spelarens färg (beror på tur) matchar pjäsens färg. om nej -> Error. om ja -> Ok. 
> ---nedan är teoretiskt---
3. Om Ok -> pjäsens möjliga drag checkas med valid_moves. Dessa presenteras till spelaren som kan välja vilket drag de vill spela. 
> valid moves bör även koll för speciella regler ?
4. draget checkas igen via valid_moves (eller dess datastruktur?). om ok exekveras det, och calls *make_move* vilket förändrar pjäsens position. *resolve_move* ser till att resolve alla konsekvenser som om en pjäs har "dödats" eller speciella moves.
5. *is_checked* och *is_checkmate* kollar statusen av kungen efter förändring. is_checkmate -> spelet är över och spelaren som senast spelade vinner. Om is_checked -> true, måste nästa spelaren skydda kungen (hur ska man göra det). Om spelet fortsätter calls *turn_tracker* som progresserar turen till nästa färg/spelare.  