nu ska vi schacka loss 💃
https://doc.rust-lang.org/reference/comments.html

# to do (in order of priority)
- [X] initialize_board (#9)
- [X] add game status struct for ending the game (#13)
- [ ] flags for king, rook, and pawns that moved 2 squares (performant way to store?) (#12)
- [ ] then, special moves: castling, en passant (#12)
- [ ] make test.rs with perft tests (?)/tests listed in test.md (#7)
## extra (ish)
- [ ] ensure no "non lib" elements are in the lib 
- [ ] bit/coordinate exchange function
- [ ] modularize checks for color/piece which is repeated all over the code (#8)
- [ ] merge sliding piece functions into one (that is, rook, bishop and queen) which varies by offsets

# bibblans flöde
1. spelaren matar ett coord. (i nuläget bit). *get_piece_at* svarar med om det finns en pjäs där; om ja, ger den vilken typ.
2. med *select_piece* kan spelaren välja en pjäs. inom den funktionen ->
> ---nedan är teoretiskt---
3. Om Ok -> pjäsens möjliga drag checkas med valid_moves. Dessa presenteras till spelaren som kan välja vilket drag de vill spela. 
> valid moves bör även koll för speciella regler ?
4. draget checkas igen via valid_moves (eller dess datastruktur?). om ok exekveras det, och calls *make_move* vilket förändrar pjäsens position. *resolve_move* ser till att resolve alla konsekvenser som om en pjäs har "dödats" eller speciella moves.
> Speciella Moves: promovering, rockad (kolla attackering??), en passant, patt (stalemate), schack/schackmatt
> fett överkurs: även kolla om det inte finns några drag kvar (om 50 drag nånting)
5. *is_checked* och *is_checkmate* kollar statusen av kungen efter förändring. is_checkmate -> spelet är över och spelaren som senast spelade vinner. Om is_checked -> true, måste nästa spelaren skydda kungen (hur ska man göra det). Om spelet fortsätter calls *turn_tracker* som progresserar turen till nästa färg/spelare.  
