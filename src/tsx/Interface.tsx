import React from 'react';
import { Settings } from '../../wasm_build';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import Typography from '@material-ui/core/Typography';
import Slider from '@material-ui/core/Slider';

const angleConvert = Math.PI / 180;

const Interface: React.FunctionComponent = () => {
    return (
        <div className="interface">
            <List disablePadding={true} dense={true}>
                <ListItem className="interface-list-item">
                    <Typography id="max-force-slider">Max Force</Typography>
                    <Slider
                        aria-labelledby="max-force-slider"
                        valueLabelDisplay="auto"
                        step={0.1}
                        min={0.1}
                        max={5}
                        defaultValue={Settings.get_max_force()}
                        onChangeCommitted={(event, value) => Settings.set_max_force(value as number)}
                    />
                </ListItem>
                <ListItem className="interface-list-item">
                    <Typography id="max-velocity-slider">Max Velocity</Typography>
                    <Slider
                        aria-labelledby="max-velocity-slider"
                        valueLabelDisplay="auto"
                        step={0.1}
                        min={0.1}
                        max={5}
                        defaultValue={Settings.get_max_velocity()}
                        onChangeCommitted={(event, value) => Settings.set_max_velocity(value as number)}
                    />
                </ListItem>
                <ListItem className="interface-list-item">
                    <Typography id="view-radius-slider">View radius</Typography>
                    <Slider
                        aria-labelledby="view-radius-slider"
                        valueLabelDisplay="auto"
                        step={1}
                        min={7}
                        max={50}
                        defaultValue={Settings.get_view_radius()}
                        onChangeCommitted={(event, value) => Settings.set_view_radius(value as number)}
                    />
                </ListItem>
                <ListItem className="interface-list-item">
                    <Typography id="weight-slider">Weight</Typography>
                    <Slider
                        aria-labelledby="weight-slider"
                        valueLabelDisplay="auto"
                        step={1}
                        min={1}
                        max={50}
                        defaultValue={Settings.get_weight()}
                        onChangeCommitted={(event, value) => Settings.set_weight(value as number)}
                    />
                </ListItem>
                <ListItem className="interface-list-item">
                    <Typography id="wander-changeable-angle-slider">Wander changeable angle</Typography>
                    <Slider
                        aria-labelledby="wander-changeable-angle-slider"
                        valueLabelDisplay="auto"
                        step={1}
                        min={10}
                        max={360}
                        defaultValue={Settings.get_wander_changeable_angle() / angleConvert}
                        onChangeCommitted={(event, value) =>
                            Settings.set_wander_changeable_angle((value as number) * angleConvert)
                        }
                    />
                </ListItem>
                <ListItem className="interface-list-item">
                    <Typography id="wander-circle-distance-slider">Wander circle distance</Typography>
                    <Slider
                        aria-labelledby="wander-circle-distance-slider"
                        valueLabelDisplay="auto"
                        step={0.5}
                        min={1}
                        max={20}
                        defaultValue={Settings.get_wander_circle_distance()}
                        onChangeCommitted={(event, value) => Settings.set_wander_circle_distance(value as number)}
                    />
                </ListItem>
                <ListItem className="interface-list-item">
                    <Typography id="wander-circle-radius-slider">Wander circle radius</Typography>
                    <Slider
                        aria-labelledby="wander-circle-radius-slider"
                        valueLabelDisplay="auto"
                        step={0.5}
                        min={1}
                        max={20}
                        defaultValue={Settings.get_wander_circle_radius()}
                        onChangeCommitted={(event, value) => Settings.set_wander_circle_radius(value as number)}
                    />
                </ListItem>
            </List>
        </div>
    );
};

export default Interface;
