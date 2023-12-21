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
pair player_pos = (0.47, 0.5);
real pa = 0.3;
pair player_dir = player_pos+2wtiles*(cos(pa), sin(pa));
real pFOV = 1.2;
pair player_left = player_pos+2wtiles*(cos(pa+pFOV), sin(pa+pFOV));
pair player_right = player_pos+2wtiles*(cos(pa-pFOV), sin(pa-pFOV));
path player_view = player_pos -- player_left -- arc(player_pos, 2wtiles, degrees(pa+pFOV), degrees(pa-pFOV)) -- player_right -- cycle;

label("Game world, side view", (0.5, 1.1), align=N, goldc);

int j = 0;

int i = 7;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+10)*wtiles)), grayc);
int i = 8;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+10)*wtiles)), grayc);


for(int i=0; i < ntiles; ++i) {
    draw(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+10)*wtiles)), linewidth(0.3));
}

draw((0,0) -- (1.1,0), arrow = Arrow);
draw((0,0) -- (0,1.1), arrow = Arrow);
label("$x$", (1.1,0), align=E);
label("$z$", (0,1.1), align=N);
label("$0$", (0,0), align=SW);
label("$1$", (0.1,0), align=S);
label("$1$", (0,1), align=W);

label("$\beta_p$", player_right, align=3N, Fill(blackc));
draw(arc(player_pos, wtiles, degrees(pa), 0));
fill(player_view, bluec+opacity(0.2));
draw(player_view, bluec);
draw(player_pos -- (player_pos.x,0), redc+linewidth(2));
draw(player_pos -- (0,player_pos.y), redc+dashed);
draw(player_pos -- (1,player_pos.y), white+dashed);
label("$x_p$", (player_pos.x,0), align=S);
label("$z_p$", (0,player_pos.y), align=W);
label("Player", player_pos, align=2S+2W, redc, Fill(blackc));
label("$\Theta$", player_left, align=N+E, bluec, Fill(blackc));
draw(player_pos -- player_dir, bluec, Arrow);
dot(player_pos, redc);

shipout(bbox(linewidth(0), Fill(blackc)));