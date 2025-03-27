import asyncio
from pyreccaster import PyReccaster, PyRecord
from p4p.nt import NTScalar
from p4p.server.asyncio import SharedPV
from p4p.server import Server


async def main():
    pv = SharedPV(nt=NTScalar('d'), initial=0.0)

    @pv.put
    def handle(pv, op):
        pv.post(op.value())
        print(f"{op.value()}")
        op.done()

    records = [
        PyRecord(name="DEV:P4P:TEST", type="ai", alias="DEV:P4P:ISIS:TEST", properties={"recordDesc": "Test ai record", "p4pVersion": "24.1.2"}),
        PyRecord(name="DEV:P4P:VAL", type="longin", alias=None, properties={"recordDesc": "Test longin record", "p4pVersion": "24.1.2"}),
    ]

    properties = {
        "ENGINEER": "P4P ENGINEER",
        "HOSTNAME": "P4P Example Machine",
    }

    with Server(providers=[{"DEV:P4P:VAL": pv}]):
        py_reccaster = await PyReccaster.setup(records, properties)
        await py_reccaster.run()


if __name__ == "__main__":
    asyncio.run(main())
