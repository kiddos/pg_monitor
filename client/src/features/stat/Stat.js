import React from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';
import TableCell from '@mui/material/TableCell';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TablePagination from '@mui/material/TablePagination';
import TableRow from '@mui/material/TableRow';
import TableSortLabel from '@mui/material/TableSortLabel';
import Typography from '@mui/material/Typography';
import Modal from '@mui/material/Modal';
import Toolbar from '@mui/material/Toolbar';
import Tooltip from '@mui/material/Tooltip';
import { visuallyHidden } from '@mui/utils';
import Highlight from 'react-highlight';
import IconButton from '@mui/material/IconButton';
import AutorenewIcon from '@mui/icons-material/Autorenew';

import 'highlight.js/styles/github.css';

import { getStatsAsync } from './statSlice';

const COLUMNS = [
  {
    id: 'userId',
    label: 'userid',
    minWidth: 50,
  },
  {
    id: 'dbId',
    label: 'dbid',
    minWidth: 50,
  },
  {
    id: 'queryId',
    label: 'queryid',
    minWidth: 60,
  },
  {
    id: 'query',
    label: 'Query',
    minWidth: 150,
    render: (value, column) => {
      return (
        <div
          style={{
            textOverflow: 'ellipsis',
            overflow: 'hidden',
            whiteSpace: 'nowrap',
            maxWidth: 200,
          }}
        >
          {value}
        </div>
      );
    },
  },
  {
    id: 'totalExecTime',
    label: 'Total Execution Time (ms)',
  },
  {
    id: 'minExecTime',
    label: 'Min Execution Time (ms)',
    minWidth: 100,
  },
  {
    id: 'maxExecTime',
    label: 'Max Execution Time (ms)',
    minWidth: 100,
  },
  {
    id: 'meanExecTime',
    label: 'Mean Execution Time (ms)',
    minWidth: 100,
  },
  {
    id: 'stddevExecTime',
    label: 'Standard Deviation Execution Time (ms)',
    minWidth: 100,
  },
  {
    id: 'row',
    label: 'row',
  },
  {
    id: 'calls',
    label: 'Calls',
  },
];

function descendingComparator(a, b, orderBy) {
  if (b[orderBy] < a[orderBy]) {
    return -1;
  }
  if (b[orderBy] > a[orderBy]) {
    return 1;
  }
  return 0;
}

function getComparator(order, orderBy) {
  return order === 'desc'
    ? (a, b) => descendingComparator(a, b, orderBy)
    : (a, b) => -descendingComparator(a, b, orderBy);
}

function EnhancedTableHead(props) {
  const { order, orderBy, onRequestSort } = props;
  const createSortHandler = property => event => {
    onRequestSort(event, property);
  };

  return (
    <TableHead>
      <TableRow>
        {COLUMNS.map(headCell => (
          <TableCell
            key={headCell.id}
            align={headCell.numeric ? 'right' : 'left'}
            padding={headCell.disablePadding ? 'none' : 'normal'}
            sortDirection={orderBy === headCell.id ? order : false}
          >
            <TableSortLabel
              active={orderBy === headCell.id}
              direction={orderBy === headCell.id ? order : 'desc'}
              onClick={createSortHandler(headCell.id)}
            >
              {headCell.label}
              {orderBy === headCell.id ? (
                <Box component="span" sx={visuallyHidden}>
                  {order === 'desc' ? 'sorted descending' : 'sorted ascending'}
                </Box>
              ) : null}
            </TableSortLabel>
          </TableCell>
        ))}
      </TableRow>
    </TableHead>
  );
}

EnhancedTableHead.propTypes = {
  onRequestSort: PropTypes.func.isRequired,
  order: PropTypes.oneOf(['asc', 'desc']).isRequired,
  orderBy: PropTypes.string.isRequired,
  rowCount: PropTypes.number.isRequired,
};

class Stat extends React.PureComponent {
  state = {
    loading: false,
    page: 0,
    pageSize: 25,
    code: null,
    order: 'desc',
    orderBy: 'totalExecTime',
  };

  fetchData = async () => {
    this.setState({ loading: true });
    await this.props.dispatch(getStatsAsync());
    this.setState({ loading: false });
  }

  componentDidMount() {
    this.fetchData();
  }

  handleChangePage = (_, page) => this.setState({ page });

  handleChangeRowsPerPage = event => {
    this.setState({ pageSize: +event.target.value, page: 0 });
  };

  handleClose = () => this.setState({ code: null });

  handleRowSelect = row => this.setState({ code: row.query });

  handleRequestSort = (_, property) => {
    const { order, orderBy } = this.state;
    const isDesc = orderBy === property && order === 'desc';
    this.setState({ order: isDesc ? 'asc' : 'desc', orderBy: property });
  };

  render() {
    const { stats } = this.props;
    const { loading, page, pageSize, code, order, orderBy } = this.state;
    return (
      <Box style={{ padding: 10 }}>
        <Modal open={!!code} onClose={this.handleClose}>
          <Box
            sx={{
              position: 'absolute',
              top: '50%',
              left: '50%',
              transform: 'translate(-50%, -50%)',
              width: 800,
              bgcolor: 'background.paper',
              border: '2px solid lightgray',
              borderRadius: 2,
              p: 4,
            }}
          >
            <Typography variant="h6" component="h2">
              SQL Statement
            </Typography>
            <Box
              sx={{
                mt: 2,
                maxHeight: 'calc(100vh - 200px)',
                overflowY: 'scroll',
              }}
            >
              <Highlight>{code}</Highlight>
            </Box>
          </Box>
        </Modal>

        <Paper style={{ padding: 20 }}>
          <Toolbar
            sx={{
              pl: { sm: 2 },
              pr: { xs: 1, sm: 1 },
            }}
          >
            <Typography variant="h6" component="div" sx={{ flex: '1 1 100%' }}>
              SQL Statements
            </Typography>
            <Tooltip title="load data...">
              <span>
                <IconButton onClick={this.fetchData} disabled={loading}>
                  <AutorenewIcon />
                </IconButton>
              </span>
            </Tooltip>
          </Toolbar>

          <TableContainer style={{ maxHeight: 'calc(100vh - 300px)' }}>
            <Table stickyHeader aria-label="sticky table">
              <EnhancedTableHead
                order={order}
                orderBy={orderBy}
                onRequestSort={this.handleRequestSort}
                rowCount={stats?.length || 0}
              />
              <TableBody>
                {stats
                  ?.slice()
                  .sort(getComparator(order, orderBy))
                  .slice(page * pageSize, (page + 1) * pageSize)
                  .map(row => {
                    return (
                      <TableRow
                        hover
                        role="checkbox"
                        tabIndex={-1}
                        key={row.queryId}
                        style={{ maxHeight: 30 }}
                        onClick={() => this.handleRowSelect(row)}
                      >
                        {COLUMNS.map(column => (
                          <TableCell key={column.id} align={column.align}>
                            {column.render
                              ? column.render(row[column.id], column)
                              : row[column.id]}
                          </TableCell>
                        ))}
                      </TableRow>
                    );
                  })}
              </TableBody>
            </Table>
          </TableContainer>
          <TablePagination
            rowsPerPageOptions={[10, 25, 100]}
            component="div"
            count={stats?.length || 0}
            rowsPerPage={pageSize}
            page={page}
            onPageChange={this.handleChangePage}
            onRowsPerPageChange={this.handleChangeRowsPerPage}
          />
        </Paper>
      </Box>
    );
  }
}

Stat.propTypes = {
  dispatch: PropTypes.func.isRequired,
};

const mapStateToProps = state => ({
  stats: state.stat.stats,
});

export default connect(mapStateToProps)(Stat);
