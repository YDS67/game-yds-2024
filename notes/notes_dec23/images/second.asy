defaultpen(fontsize(10pt));

settings.render = 16;
unitsize(5cm);

int ntiles = 10;
real wtiles = 1.0/ntiles;
pair player_pos = (0.37, 0.5);
real pa = 0.2;
pair player_dir = player_pos+2wtiles*(cos(pa), sin(pa));
real pFOV = 1.3;
pair player_left = player_pos+2wtiles*(cos(pa+pFOV), sin(pa+pFOV));
pair player_right = player_pos+2wtiles*(cos(pa-pFOV), sin(pa-pFOV));
path player_view = player_pos -- player_left -- arc(player_pos, 2wtiles, degrees(pa+pFOV), degrees(pa-pFOV)) -- player_right -- cycle;

label("Game world, side view", (0.5, 1.1), align=N);

int j = 0;

int i = 6;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+10)*wtiles)), gray);
int i = 7;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+10)*wtiles)), gray);


for(int i=0; i < ntiles; ++i) {
    draw(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+10)*wtiles)), darkgray);
}

draw((0,0) -- (1.1,0), arrow = Arrow);
draw((0,0) -- (0,1.1), arrow = Arrow);
label("$x$", (1.1,0), align=E);
label("$z$", (0,1.1), align=N);
label("$0$", (0,0), align=SW);
label("$1$", (0.1,0), align=S);
label("$1$", (0,1), align=W);

fill(player_view, purple+opacity(0.2));
draw(player_view, purple);
draw(player_pos -- (player_pos.x,0), red+linewidth(2));
draw(player_pos -- (0,player_pos.y), red+dashed);
label("$x_p$", (player_pos.x,0), align=S);
label("$z_p$", (0,player_pos.y), align=W);
label("P", player_pos, align=2S+2W, Fill(white));
label("FOV vertical", player_pos, align=14N+E, fontsize(8pt), Fill(white));
draw(player_pos -- player_dir, purple, Arrow);
dot(player_pos, red);