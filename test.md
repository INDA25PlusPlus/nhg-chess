vanliga:
- enhetstester
- integrationstester
- end-2-end tester
- bottom-up

andra:
- *regression testing*: bygger upp tester under gång. varje buggfix, ett test.
- *fuzz testing*: släng slumpmässigt input på programmet. kan göras t.ex. via **libFuzzer**
- *stress testing*: testa hur en applikation hanterar hög belastning.
- *benchmarking*: testar prestanda på applikation. 
- *manuella tester*: görs manuellt (duh). t.ex. med grafik.

tester i rust:
- #[(test)]
- #[should_panic]

automatiska teser:
- github actions; kan synkas med PR.