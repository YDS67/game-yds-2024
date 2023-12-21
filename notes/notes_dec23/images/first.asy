defaultpen(fontsize(10pt));

settings.render = 16;
unitsize(5cm);

pen blackc = RGB(40,40,40);
pen whitec = RGB(245,245,245);
pen grayc = RGB(205,205,205);
pen bluec = RGB(0,146,204);
pen redc = RGB(240,51,51);
pen goldc = RGB(207,151,58);
defaultpen(whitec);

int ntiles = 10;
real wtiles = 1.0/ntiles;
pair player_pos = (0.47, 0.33);
real pa = 1;
pair player_dir = player_pos+2.7wtiles*(cos(pa), sin(pa));
real pFOV = 0.8;
pair player_left = player_pos+2.7wtiles*(cos(pa+pFOV), sin(pa+pFOV));
pair player_right = player_pos+2.7wtiles*(cos(pa-pFOV), sin(pa-pFOV));
path player_view = player_pos -- player_left -- arc(player_pos, 2.7wtiles, degrees(pa+pFOV), degrees(pa-pFOV)) -- player_right -- cycle;


label("Game world, top view", (0.5, 1.1), align=N, goldc);

int i = 7;
int j = 5;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), grayc);
int i = 7;
int j = 6;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), grayc);
int i = 8;
int j = 5;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), grayc);
int i = 7;
int j = 7;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), grayc);

for(int i=0; i < ntiles; ++i) {
for(int j=0; j < ntiles; ++j) {
    draw(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), linewidth(0.3));
}
}

defaultpen(whitec);

draw((0,0) -- (1.1,0), arrow = Arrow);
draw((0,0) -- (0,1.1), arrow = Arrow);
label("$x$", (1.1,0), align=E);
label("$y$", (0,1.1), align=N);
label("$0$", (0,0), align=SW);
label("$1$", (0.1,0), align=S);
label("$1$", (0,0.1), align=W);

label("$\alpha_p$", player_pos + 1.5wtiles*(cos(pa/2), sin(pa/2)), Fill(blackc));
draw(arc(player_pos, wtiles, degrees(pa), 0));

fill(player_view, bluec+opacity(0.2));
draw(player_view, bluec);
draw(player_pos -- (player_pos.x,0), redc+dashed);
draw(player_pos -- (0,player_pos.y), redc+dashed);
draw(player_pos -- (1,player_pos.y), white+dashed);

label("$x_p$", (player_pos.x,0), align=S);
label("$y_p$", (0,player_pos.y), align=W);
label("Player", player_pos, align=2S+2W, redc, Fill(blackc));
label("$\Phi$", player_left, align=2N+2E, bluec, Fill(blackc));
draw(player_pos -- player_dir, bluec, Arrow);
dot(player_pos, redc);

shipout(bbox(linewidth(0), Fill(blackc)));