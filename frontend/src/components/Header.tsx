import {
    AppBar,
    Box,
    Container,
    Link,
    Toolbar,
    Typography,
} from "@mui/material";
import GitHub from "@mui/icons-material/GitHub";

export default function Header() {
    return (
        <AppBar position="static">
            <Container maxWidth="xl">
                <Toolbar>
                    <Typography variant="h6">Laskea</Typography>
                    <Box sx={{ ml: "auto" }}>
                        <Link href="#">
                            <GitHub sx={{ color: "black" }} fontSize="large" />
                        </Link>
                    </Box>
                </Toolbar>
            </Container>
        </AppBar>
    );
}
