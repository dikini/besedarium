use playground::*;

type WhitespaceChoice = tchoice!(  Http  ;
    TInteract<  Http , TClient , Message , TEnd< Http > > ,
    TInteract< Http , TServer , Response , TEnd< Http > >
);

type WhitespacePar = tpar!(  Http  ;
    TInteract<  Http , TClient , Message , TEnd< Http > > ,
    TInteract< Http , TServer , Response , TEnd< Http > >
);
