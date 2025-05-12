use besedarium::*;

type WhitespaceChoice = tchoice!(  Http  ;
    TInteract<  Http , EmptyLabel , TClient , Message , TEnd< Http , EmptyLabel > > ,
    TInteract< Http , EmptyLabel , TServer , Response , TEnd< Http , EmptyLabel > >
);

type WhitespacePar = tpar!(  Http  ;
    TInteract<  Http , EmptyLabel , TClient , Message , TEnd< Http , EmptyLabel > > ,
    TInteract< Http , EmptyLabel , TServer , Response , TEnd< Http , EmptyLabel > >
);
