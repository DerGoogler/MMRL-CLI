import React from "react"
import { Page, Toolbar } from "@mmrl/ui"
import { useActivity, useNativeFileStorage, useStrings } from "@mmrl/hooks"
import {
    ArrowBackIosRounded,
    DeleteRounded,
    AddRounded
} from "@mui/icons-material";
import {
    List,
    ListItem,
    ListItemText,
    ListSubheader,
    IconButton,
    Card,
    CardContent,
    Typography,
    Grid,
    Dialog,
    DialogActions,
    DialogContent,
    DialogContentText,
    DialogTitle,
    TextField,
    Button,
} from "@mui/material"

const info = include("info.json")

export default () => {
    const [repos, setRepos] = useNativeFileStorage("/data/adb/mmrl/repos.json", [
        "https://raw.githubusercontent.com/ya0211/magisk-modules-alt-repo/main/json/modules.json",
        "https://gr.dergoogler.com/gmr/json/modules.json"
    ], { loader: "json" })

    const { context } = useActivity()
    const { strings } = useStrings()


    const [open, setOpen] = React.useState(false)
    const [repoLink, setRepoLink] = React.useState("")
    const handleDialogOpen = () => setOpen(true)
    const handleDialogClose = () => { setOpen(false), setRepoLink("") }
    const handleRepoLinkChange = (e) => setRepoLink(e.target.value)


    const renderToolbar = React.useCallback(() => {
        return (
            <Toolbar
                modifier="noshadow"
                sx={{
                    background: "#ba420e",
                    background: "linear-gradient(22deg, #a63012 0%, #fc9e58 100%)",
                }}>
                <Toolbar.Left>
                    <Toolbar.Button icon={ArrowBackIosRounded} onClick={context.popPage} />
                </Toolbar.Left>
                <Toolbar.Center>Command line interface config</Toolbar.Center>
                <Toolbar.Right>
                    <Toolbar.Button icon={AddRounded} onClick={handleDialogOpen} />
                </Toolbar.Right>
            </Toolbar>
        )
    }, [])


    return (
        <Page renderToolbar={renderToolbar}>
            <Card sx={{ m: 1 }}>
                <CardContent>
                    <Grid container rowSpacing={0} columnSpacing={0} fullWidth>
                        <Grid item xs={5}>
                            <Typography variant="caption">Author:</Typography>
                        </Grid>
                        <Grid item xs={5}>
                            <Typography variant="caption">{info.author}</Typography>
                        </Grid>
                        <Grid item xs={5}>
                            <Typography variant="caption">Version:</Typography>
                        </Grid>
                        <Grid item xs={5}>
                            <Typography variant="caption">{info.version} ({info.versionCode})</Typography>
                        </Grid>
                        <Grid item xs={5}>
                            <Typography variant="caption">Rust version:</Typography>
                        </Grid>
                        <Grid item xs={5}>
                            <Typography variant="caption">{info.rustVersion}</Typography>
                        </Grid>
                        <Grid item xs={5}>
                            <Typography variant="caption">Build date:</Typography>
                        </Grid>
                        <Grid item xs={5}>
                            <Typography variant="caption">{info.buildDate}</Typography>
                        </Grid>
                    </Grid>
                </CardContent>
            </Card>

            <List subheader={<ListSubheader>Installed repositories</ListSubheader>}>
                {repos.map((repo) => {
                    const handleDelete = () => {
                        setRepos((rep) => rep.filter((remv) => remv != repo))
                    }
                    return (
                        <ListItem secondaryAction={
                            <IconButton edge="end" onClick={handleDelete}>
                                <DeleteRounded />
                            </IconButton>
                        }>
                            <ListItemText primary={repo} />
                        </ListItem>
                    )
                })}
            </List>


            <Dialog open={open} onClose={handleDialogOpen}>
                <DialogTitle>{strings("add_repository")}</DialogTitle>
                <DialogContent>
                    <DialogContentText>{strings("add_repository_description")}</DialogContentText>
                    <TextField
                        autoFocus
                        name="repo_link"
                        fullWidth
                        margin="dense"
                        type="text"
                        label={"Modules link"}
                        value={repoLink}
                        variant="outlined"
                        onChange={handleRepoLinkChange}
                    />
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleDialogClose}>{strings("cancel")}</Button>
                    <Button
                        onClick={() => {
                            if (!repos.some((r) => r === repoLink)) {
                                setRepos((p) => [...p, repoLink])
                                handleDialogClose()
                            }
                        }}>
                        {strings("add")}
                    </Button>
                </DialogActions>
            </Dialog>
        </Page>
    )
}