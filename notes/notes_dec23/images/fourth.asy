defaultpen(fontsize(10pt));

settings.render = 16;
unitsize(5cm);

pen blackc = RGB(40,40,40);
pen whitec = RGB(245,245,245);
pen grayc = RGB(205,205,205);
pen bluec = RGB(0,146,204);
pen redc = RGB(240,51,51);
pen greenc = RGB(34, 175, 75);
pen goldc = RGB(207,151,58);
defaultpen(whitec);

int ntiles = 10;
real wtiles = 1.0/ntiles;
pair player_pos = (0.17, 0.5);
real pa = 0.5;
pair player_dir = player_pos+5wtiles*(cos(pa), sin(pa));
real pFOV = 0.8;
pair player_left = player_pos+5wtiles*(cos(pa+pFOV), sin(pa+pFOV));
pair player_right = player_pos+5wtiles*(cos(pa-pFOV), sin(pa-pFOV));
path player_view = player_pos -- player_left -- arc(player_pos, 5wtiles, degrees(pa+pFOV), degrees(pa-pFOV)) -- player_right -- cycle;

pair target_pos = (0.8,0.95);


label("Vertical projection", (0.5, 1.1), align=N, goldc);

defaultpen(whitec);

draw((0,0) -- (1.1,0), arrow = Arrow);
draw((0,0) -- (0,1.1), arrow = Arrow);
label("$x$", (1.1,0), align=E);
label("$z$", (0,1.1), align=N);

//draw(player_left -- player_right, red);

label("$\beta_p$", player_pos + 2wtiles*(cos(pa/2), sin(pa/2)), Fill(blackc));
draw(arc(player_pos, wtiles, degrees(pa), 0));

fill(player_view, bluec+opacity(0.2));
draw(player_view, bluec);
draw(player_pos -- (player_pos.x,0), redc+dashed);
draw(player_pos -- (0,player_pos.y), redc+dashed);
draw(player_pos -- (1,player_pos.y), white+dashed);

label("$x_p$", (player_pos.x,0), align=S, redc);
label("$z_p$", (0,player_pos.y), align=W, redc);
label("P", player_pos, align=2S+2W, redc, Fill(blackc));
label("$\Theta$", player_left, align=2N+2E, bluec, Fill(blackc));
label("$R$", player_right, align=2S+2W, bluec, Fill(blackc));
draw(player_pos -- player_dir, bluec, Arrow);
dot(player_pos, redc);

dot(target_pos, greenc);
label("$x_t,z_t$", target_pos, align=E, greenc, Fill(blackc));
draw(player_pos -- target_pos, greenc, Arrow);

label("$\theta$", player_pos, greenc, align=2N+W);
draw(arc(player_pos, wtiles, degrees(pa+pFOV), degrees(atan2(target_pos.y-player_pos.y,target_pos.x-player_pos.x))), greenc);

shipout(bbox(linewidth(0), Fill(blackc), xmargin=0, ymargin=0));