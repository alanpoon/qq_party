import React,{useState} from 'react';
import './App.css';

import AutomationTab from './components/automation_tab';
import Grid from '@material-ui/core/Grid';
import Paper from '@material-ui/core/Paper';
import { makeStyles } from '@material-ui/core/styles';
const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
    padding: theme.spacing(2),
  },
  paper: {
    padding: theme.spacing(3),
    textAlign: 'center',
    color: theme.palette.text.secondary,
  }
}));

export default function App() {
  
  const classes = useStyles();
  
  return (
    <div className={classes.root}>
      <a href="/">home</a>
      <Paper>
      <Grid container spacing={1}>
        <Grid item xs={9}>
          <Paper className={classes.paper}>
            <AutomationTab/>
          </Paper>
        </Grid>
      </Grid>
      </Paper>
    </div>          
  );
  
}
